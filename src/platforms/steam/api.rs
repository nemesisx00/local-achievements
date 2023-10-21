#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use std::io::ErrorKind;
use ::anyhow::Result;
use ::reqwest::Client;
use serde::de::DeserializeOwned;
use crate::error;

use super::data::{AuthData, ResponseGetPlayerSummaries};

#[derive(Clone, Debug, Default)]
pub struct Api
{
	pub auth: AuthData,
	pub client: Client,
}

impl Api
{
	const BaseUrl: &str = "https://api.steampowered.com/";
	
	const Service_User: &str = "ISteamUser/";
	
	const Endpoint_GetPlayerSummaries: &str = "GetPlayerSummaries/v0002/";
	
	const Parameter_Format: &str = "format";
	const Parameter_Key: &str = "key";
	const Parameter_SteamIds: &str = "steamids";
	
	const Format_Json: &str = "json";
	
	pub fn new(auth: AuthData) -> Result<Self>
	{
		let client = Client::new();
		
		return match auth.validate()
		{
			true => Ok(Self { auth, client, }),
			false => Err(error!(ErrorKind::InvalidInput)),
		};
	}
	
	pub async fn getPlayerSummaries(&self) -> Result<ResponseGetPlayerSummaries>
	{
		if self.auth.validate()
		{
			let mut parameters = HashMap::<String, String>::new();
			parameters.insert(Self::Parameter_Key.into(), self.auth.key.clone());
			parameters.insert(Self::Parameter_SteamIds.into(), self.auth.id.clone());
			
			let url = self.buildUrl(Self::Service_User, Self::Endpoint_GetPlayerSummaries);
			let response = self.get::<ResponseGetPlayerSummaries>(url, parameters)
				.await?;
			
			return Ok(response);
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
	
	fn buildUrl(&self, service: &str, endpoint: &str) -> String
	{
		return format!("{}{}{}", Self::BaseUrl, service, endpoint);
	}
	
	async fn get<T>(&self, url: String, parameters: HashMap<String, String>) -> Result<T>
		where T: DeserializeOwned
	{
		let mut params = format!("?{}={}", Self::Parameter_Format, Self::Format_Json);
		for (k, v) in parameters
		{
			params = format!("{}&{}={}", params, k, v);
		}
		
		let requestUrl = format!("{}{}", url, params);
		let response = self.client.get(requestUrl)
			.send()
			.await?
			.json::<T>()
			.await?;
		
		return Ok(response);
	}
	
	/*
	async fn post<T>(&self, url: String, parameters: HashMap<String, String>) -> Result<T>
		where T: DeserializeOwned
	{
		let mut params = parameters.clone();
		params.insert("format".into(), "json".into());
		
		let response = self.client.post(url)
			.form(&params)
			.send()
			.await?
			.json::<T>()
			.await?;
		
		return Ok(response);
	}
	*/
}
