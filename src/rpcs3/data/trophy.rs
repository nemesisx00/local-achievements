use anyhow::{anyhow, Result};
use chrono::{TimeZone, Utc, offset::LocalResult};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::constants::Format_ChronoDateTime;
use crate::rpcs3::platform::data::conf::TrophyMetadata;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Trophy
{
	#[serde(default)]
	pub detail: String,
	
	#[serde(default)]
	pub grade: TrophyGrade,
	
	#[serde(default)]
	pub hidden: bool,
	
	pub id: u64,
	
	#[serde(default)]
	pub name: String,
	
	#[serde(default)]
	pub platinumRelevance: bool,
	
	#[serde(default)]
	pub unlocked: bool,
	
	/// The timestamp, in microseconds, when the trophy was unlocked.
	#[serde(default)]
	pub unlockedTimestamp: Option<u64>,
}

impl From<TrophyMetadata> for Trophy
{
	fn from(value: TrophyMetadata) -> Self
	{
		return Self
		{
			detail: value.detail.to_owned(),
			grade: value.ttype.into(),
			hidden: value.hidden == TrophyMetadata::HiddenTrue,
			id: value.id.to_owned() as u64,
			name: value.name.to_owned(),
			platinumRelevance: value.pid >= 0,
			..Default::default()
		};
	}
}

impl PartialOrd for Trophy
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return self.id.partial_cmp(&other.id);
	}
}

impl Trophy
{
	pub fn formatUnlockedTimestamp(&self) -> Result<String>
	{
		if let Some(timestamp) = self.unlockedTimestamp
		{
			return match Utc.timestamp_micros(timestamp as i64)
			{
				LocalResult::Ambiguous(earliest, _) => Ok(earliest.format(Format_ChronoDateTime).to_string()),
				LocalResult::Single(dt) => Ok(dt.format(Format_ChronoDateTime).to_string()),
				LocalResult::None => Err(anyhow!("Error parsing timestamp value: {}", timestamp)),
			}
		}
		
		return Err(anyhow!("Trophy not unlocked"));
	}
	
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut trophy = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "detail")
		{
			if let Value::String(inner) = value
			{
				trophy.detail = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "grade")
		{
			if let Value::String(inner) = value
			{
				trophy.grade = match inner.as_str()
				{
					"Bronze" => TrophyGrade::Bronze,
					"Gold" => TrophyGrade::Gold,
					"Platinum" => TrophyGrade::Platinum,
					"Silver" => TrophyGrade::Silver,
					_ => TrophyGrade::Unknown,
				};
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "hidden")
		{
			if let Value::Bool(inner) = value
			{
				trophy.hidden = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					trophy.id = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				trophy.name = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "platinumRelevance")
		{
			if let Value::Bool(inner) = value
			{
				trophy.platinumRelevance = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "unlocked")
		{
			if let Value::Bool(inner) = value
			{
				trophy.unlocked = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "unlockedTimestamp")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					trophy.unlockedTimestamp = Some(number);
				}
			}
		}
		
		return match trophy.id
		{
			0 => None,
			_ => Some(trophy)
		};
	}
	
	pub fn update(&mut self, other: &Self)
	{
		self.detail = other.detail.to_owned();
		self.grade = other.grade;
		self.hidden = other.hidden;
		self.id = other.id;
		self.name = other.name.to_owned();
		self.platinumRelevance = other.platinumRelevance;
		self.unlocked = other.unlocked;
		self.unlockedTimestamp = other.unlockedTimestamp;
	}
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub enum TrophyGrade
{
	#[default]
	Unknown,
	Platinum,
	Gold,
	Silver,
	Bronze,
}

impl From<u32> for TrophyGrade
{
	fn from(value: u32) -> Self
	{
		return (value as u64).into();
	}
}

impl From<u64> for TrophyGrade
{
	fn from(value: u64) -> Self
	{
		return match value
		{
			1 => Self::Platinum,
			2 => Self::Gold,
			3 => Self::Silver,
			4 => Self::Bronze,
			_ => Self::default(),
		};
	}
}

impl From<String> for TrophyGrade
{
	fn from(value: String) -> Self
	{
		return value.as_str().into();
	}
}

impl From<&str> for TrophyGrade
{
	fn from(value: &str) -> Self
	{
		return match value
		{
			"B" => Self::Bronze,
			"G" => Self::Gold,
			"P" => Self::Platinum,
			"S" => Self::Silver,
			_ => Self::default(),
		};
	}
}

impl TrophyGrade
{
	pub fn points(&self) -> u64
	{
		return match self
		{
			Self::Bronze => 15,
			Self::Silver => 30,
			Self::Gold => 90,
			Self::Platinum => 180,
			_ => 0,
		};
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(success: bool) -> Map<String, Value>
	{
		let mut map = Map::new();
		
		map.insert("detail".into(), "The detail".into());
		map.insert("grade".into(), "Silver".into());
		map.insert("hidden".into(), true.into());
		
		if success
		{
			map.insert("id".into(), 21.into());
		}
		
		map.insert("name".into(), "The name".into());
		map.insert("platinumRelevance".into(), true.into());
		map.insert("unlocked".into(), true.into());
		map.insert("unlockedTimestamp".into(), 333.into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let mut map = buildMap(false);
		let fail = Trophy::parseJsonMap(&map);
		assert_eq!(fail, None);
		
		map = buildMap(true);
		let success = Trophy::parseJsonMap(&map);
		assert_ne!(success, None);
		
		let trophy = success.unwrap();
		assert_eq!(trophy.detail, "The detail".to_string());
		assert_eq!(trophy.grade, TrophyGrade::Silver);
		assert_eq!(trophy.hidden, true);
		assert_eq!(trophy.id, 21);
		assert_eq!(trophy.name, "The name".to_string());
		assert_eq!(trophy.platinumRelevance, true);
		assert_eq!(trophy.unlocked, true);
		assert_eq!(trophy.unlockedTimestamp, Some(333));
	}
}
