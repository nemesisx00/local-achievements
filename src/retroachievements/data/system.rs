use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::constants::TheString;
use crate::retroachievements::platform::{GameMetadata, Payload_GetGameInfo};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct System
{
	pub id: u64,
	
	#[serde(default)]
	pub name: String,
}

impl From<GameMetadata> for System
{
	fn from(value: GameMetadata) -> Self
	{
		return Self
		{
			id: value.ConsoleID,
			name: value.ConsoleName.to_owned(),
		};
	}
}

impl From<Payload_GetGameInfo> for System
{
	fn from(value: Payload_GetGameInfo) -> Self
	{
		return Self
		{
			id: value.ConsoleID,
			name: value.ConsoleName.to_owned(),
		};
	}
}

impl PartialOrd for System
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase());
	}
}

impl System
{
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut system = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					system.id = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(string) = value
			{
				system.name = string.to_owned();
			}
		}
		
		return match system.id
		{
			0 => None,
			_ => Some(system),
		};
	}
	
	pub fn sortName(&self) -> String
	{
		return match self.name.starts_with(TheString)
		{
			true => {
				let mut the = self.name.clone();
				let name = the.split_off(TheString.len());
				format!("{}, {}", name, the.trim())
			},
			
			false => self.name.to_owned(),
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
			map.insert("id".into(), 5.into());
		}
		
		map.insert("name".into(), "The name".into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let mut map = buildMap(false);
		let fail = System::parseJsonMap(&map);
		assert_eq!(fail, None);
		
		map = buildMap(true);
		let success = System::parseJsonMap(&map);
		assert_ne!(success, None);
		
		let system = success.unwrap();
		assert_eq!(system.id, 5);
		assert_eq!(system.name, "The name".to_string());
	}
}
