#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use ::anyhow::Result;
use ::serde::{Deserialize, Serialize};
use super::data::AuthObject;

//TODO: The API is temporarily disabled for third-party users. See https://github.com/RetroAchievements/retroachievements-api-js/issues/46

#[derive(Clone, Debug, Default)]
pub struct Api
{
	pub auth: AuthObject,
}

impl Api
{
	const BaseUrl: &str = "https://retroachievements.org/API/";
	const Endpoint_GetUserRecentlyPlayedGames: &str = "API_GetUserRecentlyPlayedGames.php";
	
	fn buildUrl(&self, endpoint: &str, parameters: Option<HashMap<String, String>>) -> String
	{
		let url = format!("{}{}?z={}&y={}", Self::BaseUrl, endpoint, self.auth.username, self.auth.key);
		
		let mut extraParams = String::new();
		if let Some(params) = parameters
		{
			for (k, v) in params
			{
				extraParams = format!("{}&{}={}", extraParams, k, v);
			}
		}
		
		return format!("{}{}", url, extraParams);
	}
	
	pub async fn getUserRecentlyPlayedGames(&self, offset: usize, count: usize) -> Result<ResponseRecentlyPlayedGames>
	{
		let mut params = HashMap::new();
		params.insert("o".into(), offset.to_string());
		params.insert("c".into(), count.to_string());
		
		let url = self.buildUrl(Self::Endpoint_GetUserRecentlyPlayedGames, Some(params));
		
		let response = reqwest::get(url)
			.await?
			.json::<ResponseRecentlyPlayedGames>()
			.await?;
		return Ok(response);
	}
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResponseRecentlyPlayedGames
{
	pub games: Vec<GameInfo>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameInfo
{
	pub gameId: usize,
	pub consoleId: usize,
	pub consoleName: String,
	pub title: String,
	pub imageIcon: String,
	pub lastPlayed: String,
	pub numPossibleAchievements: usize,
	pub possibleScore: usize,
	pub numAchieved: usize,
	pub scoreAchieved: usize,
	pub numAchievedHardcore: usize,
	pub scoreAchievedHardcore: usize,
}

#[cfg(test)]
mod tests
{
    use super::*;
	
	#[test]
    fn BuildUrl()
	{
		let username = "username";
		let key = "12345678901234567890123456789012";
		
		let auth = AuthObject { username: username.into(), key: key.into() };
		let instance = Api { auth };
		
		let result = instance.buildUrl(Api::Endpoint_GetUserRecentlyPlayedGames, None);
		let expected = format!("https://retroachievements.org/API/API_GetUserRecentlyPlayedGames.php?z={}&y={}", username, key);
		
		assert_eq!(expected, result);
	}
	
    #[test]
    fn BuildUrlWithParams()
	{
		let username = "username";
		let key = "12345678901234567890123456789012";
		let offset = 20;
		let count = 5;
		
		let auth = AuthObject { username: username.into(), key: key.into() };
		let instance = Api { auth };
		let mut params = HashMap::with_capacity(2);
		params.insert("c".into(), count.to_string());
		params.insert("o".into(), offset.to_string());
		
		let result = instance.buildUrl(Api::Endpoint_GetUserRecentlyPlayedGames, Some(params.clone()));
		
		let startsWith = format!("https://retroachievements.org/API/API_GetUserRecentlyPlayedGames.php?z={}&y={}&", username, key);
		assert!(result.starts_with(startsWith.as_str()));
		
		for (k, v) in params
		{
			let expected = format!("{}={}", k, v);
			assert!(result.contains(expected.as_str()));
		}
	}
}
