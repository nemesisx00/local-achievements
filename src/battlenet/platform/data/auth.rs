use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct BattleNetAuth
{
	/// The user's API Client ID
	clientId: String,
	/// The user's API Client Secret
	clientSecret: String,
}

impl Debug for BattleNetAuth
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		return write!(f, "BattleNetAuth Redacted");
	}
}

impl Display for BattleNetAuth
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		return write!(f, "BattleNetAuth Redacted");
	}
}

impl BattleNetAuth
{
	pub const ClientIdKey: &str = "battleNetClientId";
	pub const ClientSecretKey: &str = "battleNetClientSecret";
	
	/// The user's API Client ID
	pub fn clientId(&self) -> &String
	{
		return &self.clientId;
	}
	
	/// The user's API Client Secret
	pub fn clientSecret(&self) -> &String
	{
		return &self.clientSecret;
	}
	
	pub fn new(id: String, secret: String) -> Self
	{
		return Self
		{
			clientId: id,
			clientSecret: secret,
		};
	}
}
