use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use super::mode::AchievementMode;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RankData
{
	/// The mode corresponding to this rank and point amount.
	#[serde(default)]
	pub mode: AchievementMode,
	
	/// The total number of points earned.
	#[serde(default)]
	pub points: u64,
	
	/// The current rank on RetroAchievements.org.
	#[serde(default)]
	pub rank: u64,
	
	/// The total users, used to create a relation for the rank.
	#[serde(default)]
	pub total: u64,
}

impl From<AchievementMode> for RankData
{
	fn from(value: AchievementMode) -> Self
	{
		return Self
		{
			mode: value,
			..Default::default()
		};
	}
}

impl RankData
{
	pub fn parseJsonMap(map: &Map<String, Value>) -> Self
	{
		let mut rankData = RankData::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "mode")
		{
			if let Value::Number(number) = value
			{
				rankData.mode = number.to_owned().into();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "points")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					rankData.points = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "rank")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					rankData.rank = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "total")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					rankData.total = uint;
				}
			}
		}
		
		return rankData;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap() -> Map<String, Value>
	{
		let mut map = Map::new();
		
		map.insert("mode".into(), (AchievementMode::Hardcore as u64).into());
		map.insert("points".into(), 250.into());
		map.insert("rank".into(), 10000.into());
		map.insert("total".into(), 30000.into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let map = buildMap();
		
		let rank = RankData::parseJsonMap(&map);
		assert_eq!(rank.mode, AchievementMode::Hardcore);
		assert_eq!(rank.points, 250);
		assert_eq!(rank.rank, 10000);
		assert_eq!(rank.total, 30000);
	}
}
