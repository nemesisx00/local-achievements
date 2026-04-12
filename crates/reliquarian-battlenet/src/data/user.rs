use anyhow::Result;
use data::enums::GamePlatforms;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::UserInfo;
use super::region::Region;
use super::starcraft2::profile::profile::ProfileStarcraft2;

/**
Profile information for a Battle.Net user.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BattleNetUser
{
	pub accountId: u64,
	pub battleTag: String,
	pub region: Region,
	pub starcraft2: Option<ProfileStarcraft2>,
}

impl RadioChannel<BattleNetUser> for GamePlatforms {}

impl BattleNetUser
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
					.find(|(k, _)| k.as_str() == "starcraft2")
				{
					if let Value::Object(inner) = value
					{
						user.starcraft2 = ProfileStarcraft2::parseJsonMapLossy(inner);
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "region")
				{
					if let Value::String(inner) = value
					{
						user.region = inner.into();
					}
				}
			},
			
			_ => {},
		}
		
		return Ok(user);
	}
	
	pub fn updateUserInfo(&mut self, userInfo: UserInfo)
	{
		self.accountId = userInfo.id;
		self.battleTag = userInfo.battletag.clone();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
	"accountId": 5,
	"battleTag": "The battle tag",
	"region": "eu",
	"starcraft2": null
}"#;
	
	#[test]
	fn parseJsonLossy()
	{
		let result = BattleNetUser::parseJsonLossy(PartialJson.into());
		assert!(result.is_ok());
		
		let user = result.unwrap();
		assert_eq!(user.accountId, 5);
		assert_eq!(&user.battleTag, "The battle tag");
		assert_eq!(user.region, Region::Europe);
		assert_eq!(user.region.realmId(), 1);
		assert_eq!(user.region.regionId(), 2);
		assert!(user.starcraft2.is_none());
	}
}
