use std::str::FromStr;
use std::sync::LazyLock;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use reqwest::Url;
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;
use tracing::error;
use crate::data::secure::setGogSession;
use crate::net::limiter::request::FileLocation;
use crate::{join, jpg};
use crate::gog::GogSession;
use crate::gog::platform::data::gameplay::Payload_Achievements;
use crate::gog::platform::data::listing::{FilteredProductsPage,
	FilteredProductsParameters};
use crate::io::{FileName_GameIcon, Path_Games};
use super::data::auth::*;
use super::data::users::UserInfo;

#[derive(Debug, Default)]
pub struct GogApi;

impl GogApi
{
	const ClientId: &str = "46899977096215655";
	const ClientSecret:& str = "9d85c43b1482497dbbce61f6e4aa173a433796eeae2ca8c5f6129f2dc4de46d9";
	
	const AuthUrl: &str = "https://auth.gog.com/auth";
	const RedirectUrl: &str = "https://embed.gog.com/on_login_success?origin=client";
	const TokenUrl: &str = "https://auth.gog.com/token";
	
	const GameplayHost: &str = "https://gameplay.gog.com/";
	const EmbedHost: &str = "https://embed.gog.com/";
	const UsersHost: &str = "https://users.gog.com/";
	
	const AchievementsEndpoint: &str = "clients/[productId]/users/[userId]/achievements";
	const FilteredProductsEndpoint: &str = "account/getFilteredProducts";
	const UsersEndpoint: &str = "users/";
	
	const GameIcon_Prefix: &str = "https:";
	const GameIcon_Suffix: &str = "_product_tile_117h_2x.jpg";
	const MediaType_Game: u64 = 1;
	
	pub const AuthCodeUrlRegex: &str = r"https\:\/\/embed\.gog\.com\/on_login_success\?origin\=client\&code\=(?P<code>.*)";
	pub const Platform: &str = "GOG";
	
	pub fn constructGameIconUrl(iconUrl: String) -> String
	{
		return format!(
			"{}{}{}",
			Self::GameIcon_Prefix,
			iconUrl,
			Self::GameIcon_Suffix
		);
	}
	
	pub fn constructGameIconLocation(gameId: u64) -> FileLocation
	{
		return FileLocation
		{
			fileName: jpg!(FileName_GameIcon),
			group: join!(Path_Games, gameId.to_string()),
			platform: Self::Platform.to_lowercase(),
		};
	}
	
	/**
	
	*/
	pub fn exchangeCodeForToken(code: String) -> Result<GogSession>
	{
		if code.is_empty()
		{
			return Err(anyhow!("[GOG] Authorization Code must not be empty!"))
		}
		
		let tokenParams = TokenAuthParameters
		{
			clientId: Self::ClientId.into(),
			clientSecret: Self::ClientSecret.into(),
			code,
			redirectUri: Self::RedirectUrl.into(),
			..Default::default()
		};
		
		let url = Url::from_str(&format!(
			"{}?{}",
			Self::TokenUrl,
			serde_url_params::to_string(&tokenParams)?
		))?;
		
		let tokenResponse = ureq::get(url.to_string())
			.call()
				.context("[GOG] Error sending request to GOG")?
			.body_mut()
			.read_json::<TokenResponse>()
				.context("[GOG] Error parsing response into JSON")?;
		
		let session: GogSession = tokenResponse.into();
		
		setGogSession(session.clone())
			.context("[GOG] Error saving session data to vault")?;
		
		return Ok(session);
	}
	
	/**
	Gets the achievements list for a product.
	*/
	pub fn getAchievements(session: &GogSession, userId: String, productId: u64) -> Result<Payload_Achievements>
	{
		let finalEndpoint = Self::AchievementsEndpoint
			.replace("[productId]", &productId.to_string())
			.replace("[userId]", &userId.clone());
		
		let url = Url::from_str(&format!(
			"{}{}",
			Self::GameplayHost,
			finalEndpoint
		))?;
		
		let response = Self::get::<Payload_Achievements>(url, session)?;
		return Ok(response);
	}
	
	/**
	Searches for products owned by the user matching the given criterias.
	
	Movies don’t support the parameters category, feature, system.
	
	### Query Parameters
    * category – Genre
    * feature – Feature
    * hiddenFlag – Show hidden products
    * language – Language
    * mediaType – Game or movie
    * page – Page number
    * search – Search string
    * sortBy – Sort order
    * system – OS
    * tags – Tags
    * totalPages – Total Pages
	*/
	pub fn getFilteredProducts(session: &GogSession, page: Option<u64>) -> Result<FilteredProductsPage>
	{
		let params = FilteredProductsParameters
		{
			mediaType: Some(Self::MediaType_Game),
			page: Some(page.unwrap_or(1)),
			..Default::default()
		};
		
		let url = Url::from_str(&format!(
			"{}{}?{}",
			Self::EmbedHost,
			Self::FilteredProductsEndpoint,
			serde_url_params::to_string(&params)?
		))?;
		
		let response = Self::get::<FilteredProductsPage>(url, session)?;
		return Ok(response);
	}
	
	/**
	Returns information about the user.
	*/
	pub fn getUserInfo(session: &GogSession) -> Result<UserInfo>
	{
		let url = Url::from_str(&format!(
			"{}{}{}",
			Self::UsersHost,
			Self::UsersEndpoint,
			session.userId()
		))?;
		
		let response = Self::get::<UserInfo>(url, session)?;
		return Ok(response);
	}
	
	/**
	Execute an HTTP GET request.
	*/
	fn get<T>(url: Url, session: &GogSession) -> Result<T>
		where T: DeserializeOwned
	{
		let response = ureq::get(url.to_string())
			.header(AUTHORIZATION, format!("Bearer {}", session.accessToken()))
			.call()
				.context("Error retrieving GOG API response")?
			.body_mut()
			.read_json::<T>()
				.context("Error parsing GOG API response as JSON")?;
		
		return Ok(response);
	}
	
	/**
	
	*/
	pub async fn openBrowserToAuthorize() -> Result<()>
	{
		let authParams = AuthParameters
		{
			clientId: Self::ClientId.into(),
			redirectUri: Self::RedirectUrl.to_owned(),
			..Default::default()
		};
		
		let authUrl = Url::from_str(&format!("{}?{}", Self::AuthUrl, serde_url_params::to_string(&authParams)?))?;
		
		//Note: HTTPS is required, due to including the 'hardened' feature flag
		if let Err(e) = webbrowser::open(authUrl.as_str())
		{
			error!("Error opening the default browser: {:?}", e);
		}
		
		return Ok(());
	}
	
	pub fn parseAuthCodeUrl(url: String) -> Option<String>
	{
		static RegexAuthCodeUrl: LazyLock<Regex> = LazyLock::new(||
			Regex::new(GogApi::AuthCodeUrlRegex)
				.unwrap()
		);
		
		return match RegexAuthCodeUrl.captures(&url)
		{
			None => None,
			Some(captures) => Some(captures["code"].to_string()),
		};
	}
	
	pub fn refreshAccessToken(refreshToken: String) -> Result<()>
	{
		if refreshToken.is_empty()
		{
			return Err(anyhow!("[GOG] Refresh Token must not be empty!"));
		}
		
		let refreshParams = TokenRefreshParameters
		{
			clientId: Self::ClientId.into(),
			clientSecret: Self::ClientSecret.into(),
			refreshToken: refreshToken,
			..Default::default()
		};
		
		let url = Url::from_str(&format!(
			"{}?{}",
			Self::TokenUrl,
			serde_url_params::to_string(&refreshParams)?
		))?;
		
		let tokenResponse = ureq::get(url.to_string())
			.call()
				.context("[GOG] Error sending request to GOG")?
			.body_mut()
			.read_json::<TokenResponse>()
				.context("[GOG] Error parsing resopnse into JSON")?;
		
		let session = tokenResponse.into();
		
		setGogSession(session)
			.context("[GOG] Error saving session data to vauld")?;
		
		return Ok(());
	}
}
