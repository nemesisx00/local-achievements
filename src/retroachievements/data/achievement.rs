use std::cmp::Ordering;
use std::io::ErrorKind;
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::error;
use crate::constants::{Format_ChronoDateTime, TheString};
use crate::retroachievements::platform::AchievementMetadata;
use super::makeRelative;
use super::mode::AchievementMode;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Achievement
{
	/// Number of users who have unlocked the achievement in Casual mode.
	#[serde(default)]
	pub awardedCasual: u64,
	
	/// Number of users who have unlocked the achievement in Hardcore mode.
	#[serde(default)]
	pub awardedHardcore: u64,
	
	/// Description of the achievement.
	#[serde(default)]
	pub description: String,
	
	/// Value denoting RetroAchievements' ordering of the achievement.
	#[serde(default)]
	pub displayOrder: u64,
	
	/// The timestamp when the user unlocked the achievement in Hardcore mode.
	#[serde(default)]
	pub earnedTimestampHardcore: Option<String>,
	
	/// The timestamp when the user unlocked the achievement in Casual mode.
	#[serde(default)]
	pub earnedTimestampCasual: Option<String>,
	
	/// Unique ID of the achievement.
	pub id: u64,
	
	/// Path to the icon image file.
	#[serde(default)]
	pub icon: String,
	
	/// Title of the achievement.
	#[serde(default)]
	pub name: String,
	
	/// The amount of points gained when unlocking the achievement.
	#[serde(default)]
	pub points: u64,
}

impl From<AchievementMetadata> for Achievement
{
	fn from(value: AchievementMetadata) -> Self
	{
		let mut instance = Self::default();
		instance.update(&value);
		return instance;
	}
}

impl PartialOrd for Achievement
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		let unlocked = self.unlocked(AchievementMode::Casual) || self.unlocked(AchievementMode::Hardcore);
		let otherUnlocked = other.unlocked(AchievementMode::Casual) || other.unlocked(AchievementMode::Hardcore);
		
		return match unlocked.partial_cmp(&otherUnlocked)
		{
			Some(c) => match c
			{
				Ordering::Greater => Some(Ordering::Less),
				Ordering::Less => Some(Ordering::Greater),
				
				Ordering::Equal => match self.sortName().partial_cmp(&other.sortName())
				{
					None => self.id.partial_cmp(&other.id),
					Some(c) => match c
					{
						Ordering::Equal => self.id.partial_cmp(&other.id),
						_ => Some(c),
					},
				},
			},
			
			None => match self.sortName().partial_cmp(&other.sortName())
			{
				None => self.id.partial_cmp(&other.id),
				Some(c) => match c
				{
					Ordering::Equal => self.id.partial_cmp(&other.id),
					_ => Some(c),
				},
			},
		};
	}
}

impl Achievement
{
	pub fn formatEarnedTimestamp(&self, mode: AchievementMode) -> Result<String>
	{
		if let Some(timestamp) = match mode {
				AchievementMode::Casual => &self.earnedTimestampCasual,
				AchievementMode::Hardcore => &self.earnedTimestampHardcore,
			}
		{
			let dt = self.parseTimestamp(timestamp)?;
			return Ok(dt.format(Format_ChronoDateTime).to_string());
		}
		
		return Err(error!(ErrorKind::NotFound));
	}
	
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut achievement = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "awardedCasual")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.awardedCasual = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "awardedHardcore")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.awardedHardcore = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "description")
		{
			if let Value::String(inner) = value
			{
				achievement.description = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "displayOrder")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.displayOrder = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "earnedTimestampCasual")
		{
			if let Value::String(inner) = value
			{
				if !inner.is_empty()
				{
					achievement.earnedTimestampCasual = Some(inner.to_owned());
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "earnedTimestampHardcore")
		{
			if let Value::String(inner) = value
			{
				if !inner.is_empty()
				{
					achievement.earnedTimestampHardcore = Some(inner.to_owned());
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.id = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "icon")
		{
			if let Value::String(inner) = value
			{
				achievement.icon = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				achievement.name = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "points")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.points = number;
				}
			}
		}
		
		return match achievement.id
		{
			0 => None,
			_ => Some(achievement),
		};
	}
	
	fn parseTimestamp(&self, value: &String) -> Result<NaiveDateTime>
	{
		return Ok(NaiveDateTime::parse_from_str(
			value.as_str(),
			"%Y-%m-%d %H:%M:%S"
		)?);
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
	
	pub fn unlocked(&self, mode: AchievementMode) -> bool
	{
		return match mode
		{
			AchievementMode::Casual => self.earnedTimestampCasual.is_some(),
			AchievementMode::Hardcore => self.earnedTimestampHardcore.is_some(),
		};
	}
	
	pub fn unlockedPercent(&self, mode: AchievementMode, distinctPlayers: u64) -> f64
	{
		return (match mode
		{
			AchievementMode::Casual => self.awardedCasual,
			AchievementMode::Hardcore => self.awardedHardcore,
		} as f64
			/ distinctPlayers as f64)
		* 100.0;
	}
	
	pub fn update(&mut self, achievement: &AchievementMetadata)
	{
		self.awardedCasual = achievement.NumAwarded;
		self.awardedHardcore = achievement.NumAwardedHardcore;
		self.description = achievement.Description.to_owned();
		self.displayOrder = achievement.DisplayOrder;
		self.earnedTimestampHardcore = achievement.DateEarnedHardcore.to_owned();
		self.earnedTimestampCasual = achievement.DateEarned.to_owned();
		self.icon = makeRelative(&achievement.BadgeName);
		self.id = achievement.ID;
		self.name = achievement.Title.to_owned();
		self.points = achievement.Points;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(successful: bool) -> Map<String, Value>
	{
		let mut map = Map::new();
		
		map.insert("awardedCasual".into(), 25.into());
		map.insert("awardedHardcore".into(), 5.into());
		map.insert("description".into(), "The description".into());
		map.insert("displayOrder".into(), 1.into());
		map.insert("earnedTimestampCasual".into(), "The timestamp".into());
		map.insert("earnedTimestampHardcore".into(), Value::Null);
		
		if successful
		{
			map.insert("id".into(), 2.into());
		}
		
		map.insert("icon".into(), "The icon".into());
		map.insert("name".into(), "The name".into());
		map.insert("points".into(), 15.into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let mut map = buildMap(false);
		let fail = Achievement::parseJsonMap(&map);
		assert_eq!(fail, None);
		
		map = buildMap(true);
		let success = Achievement::parseJsonMap(&map);
		assert_ne!(success, None);
		
		let achievement = success.unwrap();
		assert_eq!(achievement.awardedCasual, 25);
		assert_eq!(achievement.awardedHardcore, 5);
		assert_eq!(achievement.description, "The description".to_string());
		assert_eq!(achievement.displayOrder, 1);
		assert_eq!(achievement.earnedTimestampCasual, Some("The timestamp".to_string()));
		assert_eq!(achievement.earnedTimestampHardcore, None);
		assert_eq!(achievement.icon, "The icon".to_string());
		assert_eq!(achievement.id, 2);
		assert_eq!(achievement.name, "The name".to_string());
		assert_eq!(achievement.points, 15);
	}
}
