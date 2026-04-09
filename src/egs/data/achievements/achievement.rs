use std::cmp::Ordering;
use std::io::ErrorKind;
use anyhow::{Result, anyhow};
use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::serde::ts_milliseconds_option;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::constants::Format_ChronoDateTime;
use crate::egs::data::achievements::strings::EgsAchievementStrings;
use crate::egs::platform::data::achievement::Achievement;
use crate::egs::platform::data::progress::PlayerAchievementContainer;
use super::super::tier::EgsAchievementTier;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct EgsAchievement
{
	#[serde(default, with = "ts_milliseconds_option")]
	pub dateUnlocked: Option<DateTime<Utc>>,
	
	#[serde(default)]
	pub hidden: bool,
	
	pub id: String,
	
	#[serde(default)]
	pub isUnlocked: bool,
	
	#[serde(default)]
	pub locked: EgsAchievementStrings,
	
	#[serde(default)]
	/**
	To work around Rust's decision to not support the `Eq` trait on floating
	point types, the value is stored as an unsigned integer. Divide by 10 to
	get the actual f64 value.
	*/
	pub rarity: u64,
	
	#[serde(default)]
	pub tier: EgsAchievementTier,
	
	#[serde(default)]
	pub unlocked: EgsAchievementStrings,
	
	#[serde(default)]
	pub xp: u64,
}

impl From<&Achievement> for EgsAchievement
{
	fn from(value: &Achievement) -> Self
	{
		return Self
		{
			hidden: value.hidden,
			id: value.name.clone(),
			locked: EgsAchievementStrings
			{
				description: value.lockedDescription.clone(),
				iconId: value.lockedIconId.clone(),
				name: value.lockedDisplayName.clone(),
			},
			rarity: (value.rarity.percent * 10.0) as u64,
			tier: EgsAchievementTier
			{
				color: value.tier.hexColor.clone(),
				name: value.tier.name.clone(),
			},
			unlocked: EgsAchievementStrings
			{
				description: value.unlockedDescription.clone(),
				iconId: value.unlockedIconId.clone(),
				name: value.unlockedDisplayName.clone(),
			},
			xp: value.XP,
			..Default::default()
		};
	}
}

impl From<&PlayerAchievementContainer> for EgsAchievement
{
	fn from(value: &PlayerAchievementContainer) -> Self
	{
		return Self
		{
			id: value.playerAchievement.achievementName.clone(),
			isUnlocked: value.playerAchievement.unlocked,
			dateUnlocked: match NaiveDateTime::parse_from_str(
				&value.playerAchievement.unlockDate,
				Self::DateUnlockedFormat
			)
			{
				Err(_) => None,
				Ok(dt) => Some(dt.and_utc()),
			},
			..Default::default()
		};
	}
}

impl PartialOrd for EgsAchievement
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return match self.isUnlocked.partial_cmp(&other.isUnlocked)
		{
			None => match self.unlocked.name.partial_cmp(&other.unlocked.name)
			{
				None => Some(Ordering::Equal),
				
				Some(ordering) => match ordering
				{
					Ordering::Equal => self.tier.partial_cmp(&other.tier),
					_ => Some(ordering),
				},
			},
			
			Some(ordering) => match ordering
			{
				Ordering::Equal => match self.unlocked.name.partial_cmp(&other.unlocked.name)
				{
					None => Some(Ordering::Equal),
					Some(ordering) => match ordering
					{
						Ordering::Equal => self.tier.partial_cmp(&other.tier),
						_ => Some(ordering),
					},
				},
				
				Ordering::Greater => Some(Ordering::Less),
				Ordering::Less => Some(Ordering::Greater),
			}
		};
	}
}

impl EgsAchievement
{
	const DateUnlockedFormat: &str = "%FT%T.%3fZ";
	
	pub fn formatEarnedTimestamp(&self) -> Result<String>
	{
		if let Some(dt) = self.dateUnlocked
		{
			return Ok(dt.format(Format_ChronoDateTime).to_string());
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
				if let Some(number) = inner.as_i64()
				{
					achievement.dateUnlocked = DateTime::from_timestamp_millis(number);
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "hidden")
		{
			if let Value::Bool(inner) = value
			{
				achievement.hidden = inner.clone();
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
			.find(|(key, _)| key.as_str() == "isUnlocked")
		{
			if let Value::Bool(inner) = value
			{
				achievement.isUnlocked = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "locked")
		{
			if let Value::Object(inner) = value
			{
				achievement.locked = EgsAchievementStrings::parseJsonMap(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "rarity")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.rarity = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "tier")
		{
			if let Value::Object(inner) = value
			{
				achievement.tier = EgsAchievementTier::parseJsonMap(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "unlocked")
		{
			if let Value::Object(inner) = value
			{
				achievement.unlocked = EgsAchievementStrings::parseJsonMap(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "xp")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.xp = number;
				}
			}
		}
		
		return match achievement.id.is_empty()
		{
			false => Some(achievement),
			true => None,
		};
	}
	
	pub fn updateMetadata(&mut self, payload: &Achievement)
	{
		self.hidden = payload.hidden;
		self.id = payload.name.clone();
		self.locked.description = payload.lockedDescription.clone();
		self.locked.iconId = payload.lockedIconId.clone();
		self.locked.name = payload.lockedDisplayName.clone();
		self.rarity = (payload.rarity.percent * 10.0) as u64;
		self.tier.color = payload.tier.hexColor.clone();
		self.tier.name = payload.tier.name.clone();
		self.unlocked.description = payload.unlockedDescription.clone();
		self.unlocked.iconId = payload.unlockedIconId.clone();
		self.unlocked.name = payload.unlockedDisplayName.clone();
		self.xp = payload.XP;
	}
	
	pub fn updateProgress(&mut self, payload: &PlayerAchievementContainer)
	{
		self.id = payload.playerAchievement.achievementName.clone();
		self.isUnlocked = payload.playerAchievement.unlocked;
		self.dateUnlocked = match NaiveDateTime::parse_from_str(
			&payload.playerAchievement.unlockDate,
			Self::DateUnlockedFormat
		)
		{
			Err(_) => None,
			Ok(dt) => Some(dt.and_utc()),
		};
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(successful: bool) -> Map<String, Value>
	{
		let mut tier = Map::new();
		tier.insert("name".into(), "The tier name".into());
		tier.insert("color".into(), "The tier color".into());
		
		let mut locked = Map::new();
		locked.insert("description".into(), "Locked description".into());
		locked.insert("name".into(), "Locked display name".into());
		locked.insert("iconId".into(), "Locked icon id".into());
		
		let mut unlocked = Map::new();
		unlocked.insert("description".into(), "The description".into());
		unlocked.insert("name".into(), "The display name".into());
		unlocked.insert("iconId".into(), "The icon id".into());
		
		let mut map = Map::new();
		map.insert("hidden".into(), true.into());
		map.insert("locked".into(), locked.into());
		map.insert("unlocked".into(), unlocked.into());
		
		if successful
		{
			map.insert("id".into(), "The id".into());
		}
		
		map.insert("rarity".into(), 69.into());
		map.insert("tier".into(), tier.into());
		map.insert("isUnlocked".into(), true.into());
		map.insert("dateUnlocked".into(), 1728611918655i64.into());
		map.insert("xp".into(), 5.into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let mut map = buildMap(false);
		let fail = EgsAchievement::parseJsonMap(&map);
		assert_eq!(fail, None);
		
		map = buildMap(true);
		let success = EgsAchievement::parseJsonMap(&map);
		assert_ne!(success, None);
		
		let achievement = success.unwrap();
		assert_eq!(achievement.dateUnlocked, Some(DateTime::from_timestamp_millis(1728611918655).unwrap()));
		assert_eq!(achievement.hidden, true);
		assert_eq!(&achievement.id, "The id");
		assert_eq!(achievement.isUnlocked, true);
		assert_eq!(&achievement.locked.description, "Locked description");
		assert_eq!(&achievement.locked.iconId, "Locked icon id");
		assert_eq!(&achievement.locked.name, "Locked display name");
		assert_eq!(achievement.rarity, 69);
		
		assert_eq!(achievement.tier, EgsAchievementTier {
			color: "The tier color".into(),
			name: "The tier name".into(),
		});
		
		assert_eq!(&achievement.unlocked.description, "The description");
		assert_eq!(&achievement.unlocked.iconId, "The icon id");
		assert_eq!(&achievement.unlocked.name, "The display name");
		assert_eq!(achievement.xp, 5);
	}
}
