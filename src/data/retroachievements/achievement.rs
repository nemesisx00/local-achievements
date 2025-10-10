use std::cmp::Ordering;
use std::io::ErrorKind;
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::error;
use crate::constants::{Format_ChronoDateTime, TheString};
use crate::data::makeRelative;
use crate::data::retroachievements::mode::AchievementMode;
use crate::platforms::retroachievements::data::AchievementMetadata;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Achievement
{
	/// Number of users who have unlocked the achievement in Casual mode.
	pub awardedCasual: usize,
	
	/// Number of users who have unlocked the achievement in Hardcore mode.
	pub awardedHardcore: usize,
	
	/// Description of the achievement.
	pub description: String,
	
	/// Value denoting RetroAchievements' ordering of the achievement.
	pub displayOrder: usize,
	
	/// The timestamp when the user unlocked the achievement in Hardcore mode.
	pub earnedTimestampHardcore: Option<String>,
	
	/// The timestamp when the user unlocked the achievement in Casual mode.
	pub earnedTimestampCasual: Option<String>,
	
	/// Unique ID of the achievement.
	pub id: usize,
	
	/// Path to the icon image file.
	pub icon: String,
	
	/// Title of the achievement.
	pub name: String,
	
	/// The amount of points gained when unlocking the achievement.
	pub points: usize,
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
