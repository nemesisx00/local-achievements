#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

/**
The data necessary to access the Steam Web API
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthData
{
	/// The user's SteamID
	pub id: String,
	/// The user's Steam Web API key
	pub key: String,
}

impl AuthData
{
	/// The filename to be used when this struct is read from, or stored to, file.
	pub const FileName: &str = "steam-auth.json";
	
	/**
	Verify that this authorization data is ready to be used.
	*/
	pub fn validate(&self) -> bool
	{
		return !String::is_empty(&self.id)
			&& !String::is_empty(&self.key);
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	
    #[test]
    fn AuthData_Validate()
	{
		let instance = AuthData { id: "Test".to_string(), key: "abcdefghijklmnopqrstuvwxyz".to_string() };
		assert!(instance.validate());
		
		let mut idFail = instance.clone();
		idFail.id = String::new();
		assert!(!idFail.validate());
		
		let mut keyFail = instance.clone();
		keyFail.key = String::new();
		assert!(!keyFail.validate());
		
		let both = AuthData::default();
		assert!(!both.validate());
	}
}
