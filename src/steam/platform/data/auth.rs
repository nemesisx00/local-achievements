use std::fmt::{Debug, Display, Formatter, Result};

/**
The data necessary to access the Steam Web API.
*/
#[derive(Clone, Default)]
pub struct SteamAuth
{
	/// The user's SteamID
	id: String,
	/// The user's Steam Web API key
	key: String,
}

impl Debug for SteamAuth
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		return write!(f, "SteamAuth Redacted");
	}
}

impl Display for SteamAuth
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		return write!(f, "SteamAuth Redacted");
	}
}

impl SteamAuth
{
	pub const UserIdSecretKey: &str = "steamId";
	pub const ApiKeySecretKey: &str = "steamApiKey";
	
	pub fn new(id: String, key: String) -> Self
	{
		return Self
		{
			id,
			key,
		};
	}
	
	pub fn id(&self) -> &String
	{
		return &self.id;
	}
	
	pub fn key(&self) -> &String
	{
		return &self.key;
	}
	
	/**
	Verify that this authorization data is ready to be used.
	*/
	pub fn validate(&self) -> bool
	{
		return !self.id.is_empty()
			&& !self.key.is_empty();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	#[test]
	fn AuthData_Validate()
	{
		let id = "Test".to_string();
		let key = "abcdefghijklmnopqrstuvwxyz".to_string();
		
		assert!(SteamAuth { id: id.clone(), key: key.clone() }.validate());
		assert!(!SteamAuth { key: key.clone(), ..Default::default() }.validate());
		assert!(!SteamAuth { id: id.clone(), ..Default::default() }.validate());
		assert!(!SteamAuth::default().validate());
	}
}
