use std::fmt::{Debug, Display, Formatter};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::auth::TokenResponse;

#[derive(Clone, Deserialize, Serialize)]
pub struct GogSession
{
	accessToken: String,
	expiresIn: u64,
	refreshToken: String,
	sessionId: String,
	timestamp: DateTime<Utc>,
	userId: String,
}

impl Debug for GogSession
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		return write!(f, "GogSession Redacted");
	}
}

impl Display for GogSession
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		return write!(f, "GogSession Redacted");
	}
}

impl From<TokenResponse> for GogSession
{
	fn from(value: TokenResponse) -> Self
	{
		return Self
		{
			accessToken: value.access_token.clone(),
			expiresIn: value.expires_in.clone(),
			refreshToken: value.refresh_token.clone(),
			sessionId: value.session_id.clone(),
			timestamp: Utc::now(),
			userId: value.user_id.clone(),
		};
	}
}

impl GogSession
{
	pub const SecretKey: &str = "gogSession";
	
	pub fn hasExpired(&self) -> bool
	{
		let now = Utc::now();
		let seconds = self.timestamp.timestamp() + self.expiresIn as i64;
		let expiration = DateTime::from_timestamp(seconds, 0);
		
		return match expiration
		{
			None => true,
			Some(expirationDatetime) => expirationDatetime.signed_duration_since(now).num_seconds() <= 0,
		};
	}
	
	pub fn accessToken(&self) -> &String
	{
		return &self.accessToken;
	}
	
	/*
	pub fn expiresIn(&self) -> u64
	{
		return self.expiresIn;
	}
	*/
	
	pub fn refreshToken(&self) -> &String
	{
		return &self.refreshToken;
	}
	
	/*
	pub fn sessionId(&self) -> &String
	{
		return &self.sessionId;
	}
	
	pub fn timestamp(&self) -> &DateTime<Utc>
	{
		return &self.timestamp;
	}
	*/
	
	pub fn userId(&self) -> &String
	{
		return &self.userId;
	}
}
