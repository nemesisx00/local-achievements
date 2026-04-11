use std::str::FromStr;
use anyhow::{anyhow, Context, Result};
use net::AuthorizationManager;
use oauth2::{ResponseType, Scope};
use reqwest::{Client, Url};
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;
use tracing::info;
use crate::data::region::Region;
use crate::secure::{getBattleNetClientAuth, setBattleNetSession};
use super::{BattleNetSession, BattleNetSettings, UserInfo};

/**
Implementation of the main Battle.Net API containing authorization endpoints.
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
	
	pub const AchievementPrefix: &str = "achievement";
	pub const _RewardPrefix: &str = "reward";
	
	const AuthResponseType: &str = "code";
	const AuthScope: &str = "openid";
	
	pub const Https: &str = "https://";
	
	const RootUriOAuth: &str = "https://oauth.battle.net";
	const RootUriOAuthChina: &str = "https://oauth.battle.net.cn";
	
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
	
	pub async fn authorize(&self, region: Region) -> Result<()>
	{
		let auth = getBattleNetClientAuth()?;
		let rootUri = self.buildRootUri(region);
		
		let mut authManager = AuthorizationManager::new(
			auth.clientId().clone(),
			auth.clientSecret().clone(),
			format!("{}{}", rootUri, Self::UriAuthorization),
			format!("{}{}", rootUri, Self::UriToken),
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
	
	fn buildRootUri(&self, region: Region) -> &'static str
	{
		return match region
		{
			Region::China => Self::RootUriOAuthChina,
			_ => Self::RootUriOAuth,
		};
	}
	
	/**
	Retrieves the user's account ID and battle tag.
	*/
	pub async fn getUserInfo(&self, session: BattleNetSession, region: Region) -> Result<UserInfo>
	{
		let url = Url::from_str(format!("{}{}", self.buildRootUri(region), Self::UriUserInfo).as_str())?;
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
						.context("[BattleNet] Error retrieving API response")?
					.json::<T>().await
						.context("[BattleNet] Error parsing API response as JSON")?;
				
				Ok(response)
			},
			
			false => Err(anyhow!("[BattleNet] Session expired!"))
		};
	}
}
