use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;
use chrono::{DateTime, Utc};
use net::Oauth2Session;
use oauth2::{AccessToken, EmptyExtraTokenFields, StandardTokenResponse,
	TokenResponse};
use oauth2::basic::BasicTokenType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct BattleNetSession
{
	accessToken: AccessToken,
	expiresIn: Duration,
	timestamp: DateTime<Utc>,
	tokenType: BasicTokenType,
}

impl Debug for BattleNetSession
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		return write!(f, "BattleNetSession Redacted");
	}
}

impl Display for BattleNetSession
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		return write!(f, "BattleNetSession Redacted");
	}
}

impl Oauth2Session for BattleNetSession
{
	fn fromTokenResult(result: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>) -> Self
	{
		return Self
		{
			accessToken: result.access_token().clone(),
			expiresIn: match result.expires_in()
			{
				None => Duration::default(),
				Some(expiresIn) => expiresIn.clone(),
			},
			timestamp: Utc::now(),
			tokenType: result.token_type().clone(),
		};
	}
}

impl BattleNetSession
{
	pub const SecretKey: &str = "battleNetSession";
	
	/**
	Test if the access token has expired yet.
	
	Returns `TRUE` is the access token is expired.
	Otherwise returns `FALSE`.
	*/
	pub fn hasExpired(&self) -> bool
	{
		let expiration = self.timestamp + self.expiresIn;
		return expiration.signed_duration_since(Utc::now()).num_seconds() <= 0;
	}
	
	pub fn accessToken(&self) -> &AccessToken
	{
		return &self.accessToken;
	}
}
