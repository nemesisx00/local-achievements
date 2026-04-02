use std::str::FromStr;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::region::Region;

/**
Profile information for a Battle.Net user.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct User
{
	pub accountId: u64,
	pub battleTag: String,
	pub profileId: String,
	pub region: Region,
}

impl User
{
	pub const FileName: &str = "battlenet.json";
	
	/**
	Parse a JSON string which does not strictly conform to the expected `User`
	data structure.
	
	This function will retain as much data as possible but will omit objects if
	they are missing required properties.
	
	Any missing properties which are not strictly required will instead be
	filled with their default values.
	
	Only returns `Err` if `json` is not valid JSON.
	*/
	pub fn parseJsonLossy(json: String) -> Result<Self>
	{
		let root = serde_json::from_str::<Value>(json.as_str())?;
		
		let mut user = Self::default();
		
		match root
		{
			Value::Object(map) => {
				
				if let Some((_, value)) = map.iter()
					.find(|(key, _)| key.as_str() == "accountId")
				{
					if let Value::Number(inner) = value
					{
						if let Some(number) = inner.as_u64()
						{
							user.accountId = number;
						}
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "battleTag")
				{
					if let Value::String(inner) = value
					{
						user.battleTag = inner.clone();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "profileId")
				{
					if let Value::String(inner) = value
					{
						user.profileId = inner.clone();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "region")
				{
					if let Value::String(inner) = value
					{
						user.region = Region::from_str(inner.as_str())?;
					}
				}
			},
			
			_ => {},
		}
		
		return Ok(user);
	}
	
	pub fn update(&mut self, accountId: u64, battleTag: &String, profileId: &String, region: Region)
	{
		self.accountId = accountId;
		self.battleTag = battleTag.clone();
		self.profileId = profileId.clone();
		self.region = region;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
	"accountId": 9876543210,
	"battleTag": "The battle tag",
	"profileId": "The profile id",
	"region": "eu"
}"#;
	
	#[test]
	fn parseJsonLossy()
	{
		let result = User::parseJsonLossy(PartialJson.into());
		assert!(result.is_ok());
		
		let user = result.unwrap();
		assert_eq!(user.accountId, 9876543210);
		assert_eq!(user.battleTag, "The battle tag".to_string());
		assert_eq!(user.profileId, "The profile id".to_string());
		assert_eq!(user.region, Region::Europe);
		assert_eq!(user.region.realmId(), 1);
		assert_eq!(user.region.regionId(), 2);
	}
}
