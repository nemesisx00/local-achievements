use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct EgsAchievementTier
{
	#[serde(default)]
	pub name: String,
	
	#[serde(default)]
	pub color: String,
}

impl PartialOrd for EgsAchievementTier
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return self.name.partial_cmp(&other.name);
	}
}

impl EgsAchievementTier
{
	pub fn parseJsonMap(map: &Map<String, Value>) -> Self
	{
		let mut tier = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				tier.name = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "color")
		{
			if let Value::String(inner) = value
			{
				tier.color = inner.clone();
			}
		}
		
		return tier;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap() -> Map<String, Value>
	{
		let mut map = Map::new();
		map.insert("name".into(), "The name".into());
		map.insert("color".into(), "The color".into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let map = buildMap();
		let set = EgsAchievementTier::parseJsonMap(&map);
		assert_eq!(set.name, "The name".to_string());
		assert_eq!(set.color, "The color".to_string());
	}
}
