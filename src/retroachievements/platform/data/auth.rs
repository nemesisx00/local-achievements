use std::fmt::{Debug, Display, Formatter, Result};

/**
The combination of username and API key used to authenticate with the
RetroAchievements API.
*/
#[derive(Clone, Default)]
pub struct RetroAchievementsAuth
{
	/**
	The case-sensitive 32 character alphanumeric key associated with the
	user's account on the RetroAchievements.org website.
	*/
	key: String,
	
	/// The user's exact username used to access the RetroAchievements.org website.
	username: String,
}

impl Debug for RetroAchievementsAuth
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		return write!(f, "RetroAchievementsAuth Redacted");
	}
}

impl Display for RetroAchievementsAuth
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		return write!(f, "RetroAchievementsAuth Redacted");
	}
}

impl RetroAchievementsAuth
{
	pub const ApiKeySecretKey: &str = "raApiKey";
	pub const UsernameSecretKey: &str = "raUsername";
	
	/// The expected length of the RetroAchievements API key.
	#[allow(unused)]
	const KeyLength: u64 = 32;
	
	pub fn new(key: String, username: String) -> Self
	{
		return Self
		{
			key,
			username,
		};
	}
	
	pub fn key(&self) -> &String
	{
		return &self.key;
	}
	
	pub fn username(&self) -> &String
	{
		return &self.username;
	}
	
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
			&& self.key.len() as u64 == Self::KeyLength;
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	
    #[test]
    fn AuthObject_IsValid()
	{
		let username = "Test".to_string();
		let key = "12345678901234567890123456789012".to_string();
		
		assert!(RetroAchievementsAuth { key: key.clone(), username: username.clone() }.isValid());
		assert!(!RetroAchievementsAuth { key: key.clone(), ..Default::default() }.isValid());
		assert!(!RetroAchievementsAuth { username: username.clone(), ..Default::default() }.isValid());
		assert!(!RetroAchievementsAuth::default().isValid());
	}
}
