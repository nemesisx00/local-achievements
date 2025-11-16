use std::cmp::Ordering;
use chrono::{MappedLocalTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::steam::platform::{GameAchievement, PlayerAchievement};
use crate::constants::Format_ChronoDateTime;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Achievement
{
	/// The human-readable description of the achievement.
	#[serde(default)]
	pub description: String,
	
	/// The percentage of users on the platform who have unlocked the achievement.
	#[serde(default)]
	pub globalPercentage: Option<String>,
	
	/// Flag denoting whether or not the details of the achievement are meant to be hidden.
	#[serde(default)]
	pub hidden: bool,
	
	/// The URL used to retrieve the locked icon.
	#[serde(default)]
	pub iconLockedUrl: String,
	
	/// The URL used to retrieve the unlocked icon.
	#[serde(default)]
	pub iconUrl: String,
	
	/// The platform-specific ID of the achievement.
	pub id: String,
	
	/// The human-readable name of the achievement.
	#[serde(default)]
	pub name: String,
	
	/// The timestamp at which the achievement was unlocked.
	#[serde(default)]
	pub timestamp: Option<u64>,
}

impl PartialOrd for Achievement
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self.unlocked().partial_cmp(&other.unlocked())
		{
			None => self.id.to_lowercase().partial_cmp(&other.id.to_lowercase()),
			
			Some(c) => match c
			{
				Ordering::Equal => self.id.to_lowercase().partial_cmp(&other.id.to_lowercase()),
				Ordering::Greater => Some(Ordering::Less),
				Ordering::Less => Some(Ordering::Greater),
			},
		};
	}
}

impl From<GameAchievement> for Achievement
{
	fn from(value: GameAchievement) -> Self
	{
		return Self
		{
			description: match value.description
			{
				None => String::default(),
				Some(d) => d,
			},
			hidden: value.hidden > 0,
			iconLockedUrl: value.icongray,
			iconUrl: value.icon,
			id: value.name,
			name: value.displayName,
			..Default::default()
		};
	}
}

impl From<PlayerAchievement> for Achievement
{
	fn from(value: PlayerAchievement) -> Self
	{
		return Self
		{
			timestamp: Some(value.unlocktime),
			..Default::default()
		};
	}
}

impl Achievement
{
	pub fn formatTimestamp(&self) -> Option<String>
	{
		return match self.timestamp
		{
			None => None,
			Some(ts) => match Utc.timestamp_millis_opt(ts as i64)
			{
				MappedLocalTime::None => None,
				MappedLocalTime::Single(dt) => Some(dt.format(Format_ChronoDateTime).to_string()),
				MappedLocalTime::Ambiguous(earliest, _latest) => Some(earliest.format(Format_ChronoDateTime).to_string()),
			},
		};
	}
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut achievement = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "description")
		{
			if let Value::String(inner) = value
			{
				achievement.description = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "globalPercentage")
		{
			if let Value::String(inner) = value
			{
				if !inner.is_empty()
				{
					achievement.globalPercentage = Some(inner.to_owned());
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "hidden")
		{
			if let Value::Bool(inner) = value
			{
				achievement.hidden = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "iconLockedUrl")
		{
			if let Value::String(inner) = value
			{
				achievement.iconLockedUrl = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "iconUrl")
		{
			if let Value::String(inner) = value
			{
				achievement.iconUrl = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::String(inner) = value
			{
				achievement.id = inner.to_owned();
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
			.find(|(key, _)| key.as_str() == "timestamp")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.timestamp = Some(number);
				}
			}
		}
		
		return match achievement.id.is_empty()
		{
			false => Some(achievement),
			true => None,
		};
	}
	
	pub fn unlocked(&self) -> bool
	{
		return self.timestamp.is_some();
	}
	
	pub fn update(&mut self, achievement: &GameAchievement)
	{
		self.description = match &achievement.description
		{
			Some(d) => d.to_owned(),
			None => String::default(),
		};
		
		self.hidden = achievement.hidden > 0;
		self.iconLockedUrl = achievement.icongray.to_owned();
		self.iconUrl = achievement.icon.to_owned();
		self.id = achievement.name.to_owned();
		self.name = achievement.displayName.to_owned();
	}
	
	pub fn updateState(&mut self, achievement: &PlayerAchievement)
	{
		if achievement.unlocktime > 0
		{
			self.timestamp = Some(achievement.unlocktime * 1000);
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(success: bool) -> Map<String, Value>
	{
		let mut map = Map::new();
		map.insert("description".into(), "The description".into());
		map.insert("globalPercentage".into(), "The percent".into());
		map.insert("hidden".into(), true.into());
		map.insert("iconLockedUrl".into(), "The icon locked url".into());
		map.insert("iconUrl".into(), "The icon url".into());
		
		if success
		{
			map.insert("id".into(), "The id".into());
		}
		
		map.insert("name".into(), "The name".into());
		map.insert("timestamp".into(), 1029384756.into());
		
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
		assert_eq!(achievement.description, "The description".to_string());
		assert_eq!(achievement.globalPercentage, Some("The percent".to_string()));
		assert_eq!(achievement.hidden, true);
		assert_eq!(achievement.iconLockedUrl, "The icon locked url".to_string());
		assert_eq!(achievement.iconUrl, "The icon url".to_string());
		assert_eq!(achievement.id, "The id".to_string());
		assert_eq!(achievement.name, "The name".to_string());
		assert_eq!(achievement.timestamp, Some(1029384756));
	}
}
