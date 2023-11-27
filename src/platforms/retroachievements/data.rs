#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

/**
The combination of username and API key used to authenticate with the
RetroAchievements API.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthObject
{
	/// The user's exact username used to access the RetroAchievements.org website.
	pub username: String,
	/**
	The case-sensitive 32 character alphanumeric key associated with the
	user's account on the RetroAchievements.org website.
	*/
	pub key: String,
}

impl AuthObject
{
	/// The filename to be used when this struct is read from, or stored to, file.
	pub const FileName: &str = "ra-auth.json";
	/// The expected length of the RetroAchievements API key.
	const KeyLength: usize = 32;
	
	/**
	Evaluates whether or not this instance is valid.
	
	Valid is defined as the username is not empty and the key is of the expected
	length, as specified in the KeyLength constant.
	
	A valid instance is ready to be submitted as a part of requests to the
	RetroAchievements API.
	*/
	pub fn isValid(&self) -> bool
	{
		return !String::is_empty(&self.username)
			&& self.key.len() == Self::KeyLength;
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	
    #[test]
    fn AuthObject_IsValid()
	{
		let instance = AuthObject { username: "Test".to_string(), key: "12345678901234567890123456789012".to_string() };
		assert!(instance.isValid());
		
		let mut keyFail = instance.clone();
		keyFail.key = "1234567890".to_string();
		assert!(!keyFail.isValid());
		
		let mut usernameFail = instance.clone();
		usernameFail.username = String::new();
		assert!(!usernameFail.isValid());
		
		let both = AuthObject::default();
		assert!(!both.isValid());
	}
}
