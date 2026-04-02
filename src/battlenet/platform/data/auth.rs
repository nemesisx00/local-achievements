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

/**
The data necessary to access the Steam Web API.
*/
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BattleNetSettings
{
	/// The port to use when constructing the Redirect URI
	pub redirectPort: u64,
}

impl Default for BattleNetSettings
{
	fn default() -> Self
	{
		return Self
		{
			redirectPort: 8080,
		};
	}
}

impl BattleNetSettings
{
	/// The filename to be used when this struct is read from, or written to, the file system.
	pub const FileName: &str = "battlenet.json";
}
