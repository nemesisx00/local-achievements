use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use anyhow::{anyhow, Result};
use oauth2::basic::{BasicClient};
use oauth2::{AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
	CsrfToken, PkceCodeChallenge, RedirectUrl, ResponseType, Scope, TokenUrl};
use reqwest::{ClientBuilder, Url};
use reqwest::header::HeaderValue;
use reqwest::redirect::Policy;
use tracing::{error, info, warn};
use crate::net::Oauth2Session;

/**
Object to contain and manage the standard OAuth2 authorization flow.
*/
pub struct AuthorizationManager
{
	authCode: Option<AuthorizationCode>,
	authUrl: AuthUrl,
	clientId: ClientId,
	clientSecret: ClientSecret,
	csrfToken: Option<CsrfToken>,
	preferredRedirectPort: Option<u64>,
	tcpListener: Option<TcpListener>,
	tokenUrl: TokenUrl,
}

impl AuthorizationManager
{			
	const SuccessHtml: &str = r#"<!DOCTYPE html>
<html>
	<head>
		<title>Platform Authentication Successful | Local Achievements</title>
		<style>
			body
			{
				color: rgb(204, 204, 204);
				background-color: rgb(35, 35, 35);
			}
			
			h1 { text-align: center; }
			p { text-align: center; }
		</style>
	</head>
	<body>
		<h1>Authentication Successful</h1>
		<p>You can close this tab and return to Local Achievements.</p>
	</body>
</html>"#;
	
	pub fn constructAuthorizationHeader(authScheme: String, accessToken: &AccessToken) -> Result<HeaderValue>
	{
		let mut headerValue = HeaderValue::from_str(format!(
			"{} {}",
			authScheme,
			accessToken.secret()
		).as_str())?;
		headerValue.set_sensitive(true);
		return Ok(headerValue);
	}
	
	pub fn new(clientId: String, clientSecret: String, authUrl: String, tokenUrl: String, redirectPort: Option<u64>) -> Result<Self>
	{
		let authUrl = AuthUrl::new(authUrl)?;
		let tokenUrl = TokenUrl::new(tokenUrl)?;
		
		return Ok(Self
		{
			authCode: None,
			authUrl,
			clientId: ClientId::new(clientId),
			clientSecret: ClientSecret::new(clientSecret),
			csrfToken: None,
			preferredRedirectPort: redirectPort,
			tcpListener: None,
			tokenUrl,
		});
	}
	
	pub async fn authorizationCodeFlow<T>(&mut self, responseType: ResponseType, scopes: Vec<Scope>) -> Result<T>
		where T: Oauth2Session
	{
		let mut out: Option<T> = None;
		self.bindListener()?;
		
		if let Some(listener) = &self.tcpListener
		{
			let localAddr = listener.local_addr()?;
			let redirectUrl = RedirectUrl::new(format!("http://{}", localAddr.to_string()))?;
			
			let client = BasicClient::new(self.clientId.clone())
				.set_client_secret(self.clientSecret.clone())
				.set_auth_uri(self.authUrl.clone())
				.set_token_uri(self.tokenUrl.clone())
				.set_redirect_uri(redirectUrl);
			
			let (pkceChallenge, pkceVerifier) = PkceCodeChallenge::new_random_sha256();
			
			let (authUrl, csrfState) = client.authorize_url(CsrfToken::new_random)
				.add_scopes(scopes)
				.set_pkce_challenge(pkceChallenge)
				.set_response_type(&responseType)
				.url();
			
			self.csrfToken = Some(csrfState);
			
			//Note: HTTPS is required, due to including the 'hardened' feature flag
			if let Err(e) = webbrowser::open(authUrl.as_str())
			{
				error!("Error opening the default browser: {:?}", e);
			}
			
			self.waitForResponse().await?;
			
			if let Some(authCode) = self.authCode.clone()
			{
				let httpClient = ClientBuilder::new()
					.redirect(Policy::none())
					.build()?;
				
				let tokenResult = client
					.exchange_code(authCode)
					.set_pkce_verifier(pkceVerifier)
					.request_async(&httpClient)
					.await?;
				
				out = Some(T::fromTokenResult(tokenResult));
			}
		}
		
		self.dropListener();
		
		return match out
		{
			None => Err(anyhow!("Authorization flow failed without error")),
			Some(session) => Ok(session),
		};
	}
	
	fn bindListener(&mut self) -> Result<()>
	{
		let uri = format!("127.0.0.1:{}", match self.preferredRedirectPort
		{
			// Note: When the port is 0, this prompts the OS to choose a random port to which to bind
			None => 0,
			Some(port) => port,
		});
		
		self.tcpListener = Some(TcpListener::bind(uri)?);
		
		if let Some(listener) = &self.tcpListener
		{
			info!("Listening on: {}", listener.local_addr()?);
		}
		
		return Ok(());
	}
	
	fn dropListener(&mut self)
	{
		self.tcpListener = None;
	}
	
	/**
	Call `TcpListener::accept()` and wait for the redirect from the login process.
	Parse the authorization code from the response.
	*/
	async fn waitForResponse(&mut self) -> Result<()>
	{
		if let Some(listener) = &self.tcpListener
		{
			match listener.accept()
			{
				Err(e) => warn!("[OAuth2] Listener::accept error: {:?}", e),
				Ok((mut stream, address)) => {
					info!("[OAuth2] Received request from: {}", address);
					
					let bufReader = BufReader::new(&stream);
					let httpRequest: Vec<_> = bufReader
						.lines()
						.map(|result| result.unwrap())
						.take_while(|line| !line.is_empty())
						.collect();
					
					if let Some(get) = httpRequest.iter()
						.find(|l| l.starts_with("GET /?"))
					{
						if let Some(parameters) = get.split_whitespace().nth(1)
						{
							//TODO: Switch this to if let Ok(url) once stabilized, to avoid putting sensitive information into the logs
							match Url::parse(&format!("http://127.0.0.1{}", parameters))
							{
								Err(e) => warn!("[OAuth2] Failed to parse the url with parameters: {:?}", e),
								Ok(url) => {
									let state = url.query_pairs()
										.find_map(|(k, v)| match k == "state"
										{
											false => None,
											true => Some(CsrfToken::new(v.into_owned())),
										});
									
									//Verify that the CSRF tokens match
									if self.csrfToken.as_ref().is_some_and(|token| state.is_some_and(|s| s.secret() == token.secret()))
									{
										self.authCode = url.query_pairs()
											.find_map(|(k, v)| match k == "code"
										{
											false => None,
											true => Some(AuthorizationCode::new(v.into_owned())),
										});
									}
								}
							}
						}
					}
					
					let response = format!(
						"HTTP/1.1 200 OK\r\n\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
						Self::SuccessHtml.len(),
						Self::SuccessHtml
					);
					stream.write_all(response.as_bytes())?;
				},
			}
		}
		
		return Ok(());
	}
}
