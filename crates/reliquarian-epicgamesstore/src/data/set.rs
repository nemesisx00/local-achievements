use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct AchievementSet
{
	pub id: String,
	
	#[serde(default)]
	pub isBase: bool,
	
	#[serde(default)]
	pub totalAchievements: u64,
	
	#[serde(default)]
	pub totalXp: u64,
}

impl PartialOrd for AchievementSet
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return self.id.partial_cmp(&other.id);
	}
}

impl AchievementSet
{
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut tier = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::String(inner) = value
			{
				tier.id = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "isBase")
		{
			if let Value::Bool(inner) = value
			{
				tier.isBase = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalAchievements")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					tier.totalAchievements = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalXp")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					tier.totalXp = number;
				}
			}
		}
		
		return match tier.id.is_empty()
		{
			false => Some(tier),
			true => None,
		};
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(successful: bool) -> Map<String, Value>
	{
		let mut map = Map::new();
		
		if successful
		{
			map.insert("id".into(), "The id".into());
		}
		
		map.insert("isBase".into(), true.into());
		map.insert("totalAchievements".into(), 42.into());
		map.insert("totalXp".into(), 69.into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let mut map = buildMap(false);
		let fail = AchievementSet::parseJsonMap(&map);
		assert_eq!(fail, None);
		
		map = buildMap(true);
		let success = AchievementSet::parseJsonMap(&map);
		assert_ne!(success, None);
		
		let set = success.unwrap();
		assert_eq!(set.id, "The id".to_string());
		assert_eq!(set.isBase, true);
		assert_eq!(set.totalAchievements, 42);
		assert_eq!(set.totalXp, 69);
	}
}
