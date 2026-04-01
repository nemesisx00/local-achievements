use serde::{Deserialize, Serialize};

/**
The data necessary to access the Steam Web API.
*/
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SteamAuth
{
	/// The user's SteamID
	pub id: String,
	/// The user's Steam Web API key
	pub key: String,
}

impl SteamAuth
{
	/// The filename to be used when this struct is read from, or written to, the file system.
	pub const FileName: &str = "steam-auth.json";
	
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
		
		assert!(SteamAuth { id: id.to_owned(), key: key.to_owned() }.validate());
		assert!(!SteamAuth { key: key.to_owned(), ..Default::default() }.validate());
		assert!(!SteamAuth { id: id.to_owned(), ..Default::default() }.validate());
		assert!(!SteamAuth::default().validate());
	}
}
