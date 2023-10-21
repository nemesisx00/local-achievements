#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use std::io::ErrorKind;
use ::anyhow::{Error, Result};
use super::data::{AuthData, ResponseGetPlayerSummaries};

#[derive(Clone, Debug, Default)]
pub struct Api
{
	pub auth: AuthData,
}

impl Api
{
	const BaseUrl: &str = "https://api.steampowered.com/";
	const Service_User: &str = "ISteamUser/";
	const Endpoint_GetPlayerSummaries: &str = "GetPlayerSummaries/v0002/";
	
	fn buildUrl(&self, service: &str, endpoint: &str, parameters: Option<HashMap<String, String>>) -> String
	{
		let url = format!("{}{}{}", Self::BaseUrl, service, endpoint);
		
		let mut params = String::new();
		if let Some(map) = parameters
		{
			for (k, v) in map
			{
				params = format!("{}&{}={}", params, k, v);
			}
		}
		
		return format!("{}?{}", url, params);
	}
	
	pub async fn getPlayerSummaries(&self) -> Result<ResponseGetPlayerSummaries>
	{
		if self.auth.validate()
		{
			let mut parameters = HashMap::new();
			parameters.insert("key".into(), self.auth.key.clone());
			parameters.insert("steamids".into(), self.auth.id.clone());
			
			let url = self.buildUrl(Self::Service_User, Self::Endpoint_GetPlayerSummaries, Some(parameters));
			let response = reqwest::get(url)
				.await?
				.json::<ResponseGetPlayerSummaries>()
				.await?;
			
			return Ok(response);
		}
	
		return Err(Error::from(std::io::Error::from(ErrorKind::InvalidInput)));
	}
}
