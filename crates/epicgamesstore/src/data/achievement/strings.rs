use ::serde::{Deserialize, Serialize};
use ::serde_json::{Map, Value};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct EgsAchievementStrings
{
	#[serde(default)]
	pub description: String,
	
	#[serde(default)]
	pub iconId: String,
	
	#[serde(default)]
	pub name: String,
}

impl PartialOrd for EgsAchievementStrings
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return self.name.partial_cmp(&other.name);
	}
}

impl EgsAchievementStrings
{
	pub fn filterForText(&self, search: &String) -> bool
	{
		return self.name.to_lowercase().contains(search)
			|| self.description.to_lowercase().contains(search);
	}
	
	pub fn parseJsonMap(map: &Map<String, Value>) -> Self
	{
		let mut strings = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "description")
		{
			if let Value::String(inner) = value
			{
				strings.description = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "iconId")
		{
			if let Value::String(inner) = value
			{
				strings.iconId = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				strings.name = inner.clone();
			}
		}
		
		return strings;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap() -> Map<String, Value>
	{
		let mut map = Map::new();
		map.insert("description".into(), "The description".into());
		map.insert("name".into(), "The name".into());
		map.insert("iconId".into(), "The icon id".into());
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let map = buildMap();
		let strings = EgsAchievementStrings::parseJsonMap(&map);
		
		assert_eq!(&strings.description, "The description");
		assert_eq!(&strings.name, "The name");
		assert_eq!(&strings.iconId, "The icon id");
	}
}
