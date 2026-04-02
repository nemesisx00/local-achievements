use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::battlenet::data::region::Region;
use crate::battlenet::platform::data::starcraft2::account::PayloadPlayer;
use crate::battlenet::platform::data::starcraft2::profile::profile::PayloadProfile;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProfileStarcraft2
{
	pub id: u64,
	pub region: Region,
	pub name: String,
	pub totalAchievementPoints: u64,
	pub totalSwarmLevel: u64,
}

impl From<PayloadPlayer> for ProfileStarcraft2
{
	fn from(value: PayloadPlayer) -> Self
	{
		return Self
		{
			id: value.profileId
				.parse::<u64>().unwrap_or_default(),
			
			name: value.name.clone(),
			region: Region::fromRegionRealm(value.regionId, value.realmId),
			..Default::default()
		};
	}
}

impl ProfileStarcraft2
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Option<Self>
	{
		let mut profile = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					profile.id = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "region")
		{
			if let Value::String(inner) = value
			{
				profile.region = inner.into();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				profile.name = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalAchievementPoints")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					profile.totalAchievementPoints = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalSwarmLevel")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					profile.totalSwarmLevel = number;
				}
			}
		}
		
		return match profile.id > 0
		{
			false => None,
			true => Some(profile),
		};
	}
	
	pub fn updateAccount(&mut self, account: PayloadPlayer)
	{
		self.id = account.profileId
			.parse::<u64>().unwrap_or_default();
		
		self.name = account.name.clone();
		self.region = Region::fromRegionRealm(account.regionId, account.realmId);
	}
	
	pub fn updateProfile(&mut self, profile: PayloadProfile)
	{
		self.totalAchievementPoints = profile.summary.totalAchievementPoints;
		self.totalSwarmLevel = profile.summary.totalSwarmLevel;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
		"id": 7,
		"region": "kr",
		"name": "The starcraft 2 name",
		"totalAchievementPoints": 20
	}
"#;
	
	#[test]
	fn parseJsonMapLossy()
	{
		let root = serde_json::from_str::<Value>(PartialJson);
		assert!(root.is_ok());
		
		let value = root.unwrap();
		assert!(value.is_object());
		
		let map = value.as_object();
		assert!(map.is_some());
		
		let result = ProfileStarcraft2::parseJsonMapLossy(&map.unwrap());
		assert!(result.is_some());
		
		let profile = result.unwrap();
		assert_eq!(profile.id, 7);
		assert_eq!(profile.region, Region::Korea);
		assert_eq!(&profile.name, "The starcraft 2 name");
		assert_eq!(profile.totalAchievementPoints, 20);
		assert_eq!(profile.totalSwarmLevel, 0);
	}
}
