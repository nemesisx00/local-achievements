use std::cmp::Ordering;
use std::io::ErrorKind;
use anyhow::{anyhow, Result};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::constants::{Format_ChronoDateTime, TheString};
use crate::gog::platform::data::gameplay::AchievementMetadata;

/**
The 
*/
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Ord, Serialize)]
pub struct GogAchievement
{
	/// 
	#[serde(default)]
	pub dateUnlocked: Option<i64>,
	
	/// 
	#[serde(default)]
	pub description: String,
	
	/// 
	#[serde(default)]
	pub id: String,
	
	/// 
	#[serde(default)]
	pub key: String,
	
	/// 
	#[serde(default)]
	pub name: String,
	
	/// 
	#[serde(default)]
	pub visible: bool,
}

impl From<AchievementMetadata> for GogAchievement
{
	fn from(value: AchievementMetadata) -> Self
	{
		let mut instance = Self::default();
		instance.update(&value);
		return instance;
	}
}

impl PartialOrd for GogAchievement
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self.dateUnlocked.partial_cmp(&other.dateUnlocked)
		{
			None => match self.sortName().partial_cmp(&other.sortName())
			{
				None => self.id.partial_cmp(&other.id),
				Some(c) => match c
				{
					Ordering::Equal => self.id.partial_cmp(&other.id),
					_ => Some(c),
				},
			},
			
			Some(c) => match c
			{
				Ordering::Equal => match self.sortName().partial_cmp(&other.sortName())
				{
					None => self.id.partial_cmp(&other.id),
					Some(c) => match c
					{
						Ordering::Equal => self.id.partial_cmp(&other.id),
						_ => Some(c),
					},
				},
				
				Ordering::Greater => Some(Ordering::Less),
				Ordering::Less => Some(Ordering::Greater),
			},
		};
	}
}

impl GogAchievement
{
	const DateUnlockedFormat: &str = "%FT%T%z";
	
	pub fn formatEarnedTimestamp(&self) -> Result<String>
	{
		if let Some(timestamp) = self.dateUnlocked
		{
			if let Some(dt) = DateTime::from_timestamp(timestamp, 0)
			{
				return Ok(dt.format(Format_ChronoDateTime).to_string());
			}
		}
		
		return Err(anyhow!(ErrorKind::NotFound));
	}
	
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut achievement = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "dateUnlocked")
		{
			if let Value::Number(inner) = value
			{
				if let Some(int) = inner.as_i64()
				{
					achievement.dateUnlocked = Some(int);
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "description")
		{
			if let Value::String(inner) = value
			{
				achievement.description = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::String(inner) = value
			{
				achievement.id = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "key")
		{
			if let Value::String(inner) = value
			{
				achievement.key = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				achievement.name = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "visible")
		{
			if let Value::Bool(inner) = value
			{
				achievement.visible = *inner;
			}
		}
		
		return match achievement.id.is_empty()
		{
			true => None,
			false => Some(achievement),
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
			
			false => self.name.clone(),
		};
	}
	
	pub fn update(&mut self, achievement: &AchievementMetadata)
	{
		self.dateUnlocked = match &achievement.date_unlocked
		{
			None => None,
			Some(s) => match DateTime::parse_from_str(&s, Self::DateUnlockedFormat)
			{
				Err(_) => None,
				Ok(dt) => Some(dt.timestamp()),
			},
		};
		
		self.description = achievement.description.clone();
		self.id = achievement.achievement_id.clone();
		self.key = achievement.achievement_key.clone();
		self.name = achievement.name.clone();
		self.visible = achievement.visible;
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
		
		map.insert("key".into(), "The key".into());
		map.insert("visible".into(), true.into());
		map.insert("name".into(), "The name".into());
		map.insert("description".into(), "The description".into());
		//TODO: Use a test string representative of actual data
		map.insert("dateUnlocked".into(), "The date unlocked".into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let mut map = buildMap(false);
		let fail = GogAchievement::parseJsonMap(&map);
		assert_eq!(fail, None);
		
		map = buildMap(true);
		let success = GogAchievement::parseJsonMap(&map);
		assert_ne!(success, None);
		
		let achievement = success.unwrap();
		assert_eq!(achievement.id, "The id".to_string());
		assert_eq!(achievement.key, "The key".to_string());
		assert_eq!(achievement.visible, true);
		assert_eq!(achievement.name, "The name".to_string());
		assert_eq!(achievement.description, "The description".to_string());
		assert_eq!(achievement.dateUnlocked, None);
	}
}
