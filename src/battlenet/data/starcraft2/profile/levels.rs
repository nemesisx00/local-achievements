use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::battlenet::platform::data::starcraft2::profile::profile::SwarmLevel;
use crate::util::truncateF32;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FactionLevel
{
	pub currentPoints: u64,
	pub level: u64,
	pub maxPoints: u64,
}

impl From<SwarmLevel> for FactionLevel
{
	fn from(value: SwarmLevel) -> Self
	{
		return Self
		{
			currentPoints: value.currentLevelPoints,
			level: value.level,
			maxPoints: value.maxLevelPoints,
		};
	}
}

impl FactionLevel
{
	pub fn percentToNextLevel(&self) -> f32
	{
		return match self.maxPoints > 0
		{
			false => 0.0,
			true => truncateF32(
				self.currentPoints as f32
					/ self.maxPoints as f32,
				2
			),
		}
	}
	
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Self
	{
		let mut level = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "currentPoints")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					level.currentPoints = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "level")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					level.level = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "maxPoints")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					level.maxPoints = number;
				}
			}
		}
		
		return level;
	}
}
