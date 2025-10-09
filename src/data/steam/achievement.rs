use std::cmp::Ordering;
use chrono::{MappedLocalTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use crate::platforms::steam::data::{GameAchievement, PlayerAchievement};
use crate::constants::Format_ChronoDateTime;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Achievement
{
	/// The human-readable description of the achievement.
	pub description: String,
	
	/// The percentage of users on the platform who have unlocked the achievement.
	pub globalPercentage: Option<String>,
	
	/// Flag denoting whether or not the details of the achievement are meant to be hidden.
	pub hidden: bool,
	
	/// The URL used to retrieve the locked icon.
	pub iconLockedUrl: String,
	
	/// The URL used to retrieve the unlocked icon.
	pub iconUrl: String,
	
	/// The platform-specific ID of the achievement.
	pub id: String,
	
	/// The human-readable name of the achievement.
	pub name: String,
	
	/// The timestamp at which the achievement was unlocked.
	pub timestamp: Option<usize>,
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
