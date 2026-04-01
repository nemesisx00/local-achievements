#![allow(unused)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct AuthParameters
{
	#[serde(rename = "client_id")]
	pub clientId: String,
	#[serde(default = "client2")]
	pub layout: String,
	#[serde(rename = "redirect_uri")]
	pub redirectUri: String,
	#[serde(default = "code")]
	#[serde(rename = "response_type")]
	pub responseType: String,
}

impl Default for AuthParameters
{
	fn default() -> Self
	{
		return Self
		{
			clientId: Default::default(),
			layout: "client2".into(),
			redirectUri: Default::default(),
			responseType: "code".into(),
		};
	}
}

#[derive(Clone, Debug, Serialize)]
pub struct TokenAuthParameters
{
	#[serde(rename = "client_id")]
	pub clientId: String,
	#[serde(rename = "client_secret")]
	pub clientSecret: String,
	pub code: String,
	#[serde(rename = "grant_type")]
	#[serde(default = "authorization_code")]
	pub grantType: String,
	#[serde(rename = "redirect_uri")]
	pub redirectUri: String,
}

impl Default for TokenAuthParameters
{
	fn default() -> Self
	{
		return Self
		{
			clientId: Default::default(),
			clientSecret: Default::default(),
			code: Default::default(),
			grantType: "authorization_code".into(),
			redirectUri: Default::default(),
		};
	}
}

#[derive(Clone, Debug, Serialize)]
pub struct TokenRefreshParameters
{
	#[serde(rename = "client_id")]
	pub clientId: String,
	#[serde(rename = "client_secret")]
	pub clientSecret: String,
	#[serde(rename = "grant_type")]
	#[serde(default = "refresh_token")]
	pub grantType: String,
	#[serde(rename = "refresh_token")]
	pub refreshToken: String,
}

impl Default for TokenRefreshParameters
{
	fn default() -> Self
	{
		return Self
		{
			clientId: Default::default(),
			clientSecret: Default::default(),
			grantType: "refresh_token".into(),
			refreshToken: Default::default(),
		};
	}
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct TokenResponse
{
	pub access_token: String,
	pub expires_in: u64,
	pub refresh_token: String,
	pub scope: String,
	pub session_id: String,
	pub token_type: String,
	pub user_id: String,
}
