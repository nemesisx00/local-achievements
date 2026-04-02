use std::str::FromStr;
use anyhow::{anyhow, Context, Result};
use oauth2::{ResponseType, Scope};
use reqwest::{Client, Url};
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;
use tracing::info;
use crate::battlenet::platform::data::settings::BattleNetSettings;
use crate::battlenet::platform::data::userinfo::UserInfo;
use crate::data::secure::{getBattleNetClientAuth, setBattleNetSession};
use crate::net::AuthorizationManager;
use super::data::session::BattleNetSession;

/**
Implementation of the main Battle.Net containing authorization endpoints.
*/
#[derive(Debug, Default)]
pub struct BattleNetApi
{
	client: Client,
	settings: BattleNetSettings,
}

impl BattleNetApi
{
	pub const Platform: &str = "BattleNet";
	
	const AuthResponseType: &str = "code";
	const AuthScope: &str = "openid";
	
	pub const Https: &str = "https://";
	const RootUriOauth: &str = "https://oauth.battle.net";
	const RootUriOauthChina: &str = "https://oauth.battle.net.cn";
	
	const UriAuthorization: &str = "/authorize";
	const UriToken: &str = "/token";
	const UriUserInfo: &str = "/userinfo";
	
	pub fn new(settings: BattleNetSettings) -> Self
	{
		return Self
		{
			settings,
			..Default::default()
		};
	}
	
	pub async fn authorize(&self) -> Result<()>
	{
		let auth = getBattleNetClientAuth()?;
		
		let mut authManager = AuthorizationManager::new(
			auth.clientId().clone(),
			auth.clientSecret().clone(),
			format!("{}{}", Self::RootUriOauth, Self::UriAuthorization),
			format!("{}{}", Self::RootUriOauth, Self::UriToken),
			Some(self.settings.redirectPort)
		)?;
		
		return match authManager.authorizationCodeFlow(
			ResponseType::new(Self::AuthResponseType.into()),
			vec![Scope::new(Self::AuthScope.into())]
		).await
		{
			Err(e) => Err(anyhow!("[Battle.Net] authorization flow failed: {:?}", e)),
			Ok(session) => {
				_ = setBattleNetSession(session)?;
				info!("[Battle.Net] authorization flow succeeded.");
				Ok(())
			},
		};
	}
	
	/**
	Retrieves the user's account ID and battle tag.
	*/
	pub async fn userInfo(&self, session: BattleNetSession) -> Result<UserInfo>
	{
		let url = Url::from_str(format!("{}{}", Self::RootUriOauth, Self::UriUserInfo).as_str())?;
		let userInfo = self.get::<UserInfo>(url, session).await?;
		return Ok(userInfo);
	}
	
	/**
	Execute an HTTP GET request.
	
	Passes the `Session`'s access token in the `AUTHORIZATION` header.
	
	Returns `Err` if the `Session` has expired.
	*/
	pub async fn get<T>(&self, url: Url, session: BattleNetSession) -> Result<T>
		where T: DeserializeOwned
	{
		return match !session.hasExpired()
		{
			true => {
				let authHeader = AuthorizationManager::constructAuthorizationHeader(
					"Bearer".to_string(),
					session.accessToken()
				)?;
				
				let response = self.client.get(url)
					.header(AUTHORIZATION, authHeader)
					.send().await
						.context("Error retrieving GOG API response")?
					.json::<T>().await
						.context("Error parsing GOG API response as JSON")?;
				
				Ok(response)
			},
			
			false => Err(anyhow!("Battle.Net session expired!"))
		};
	}
}
