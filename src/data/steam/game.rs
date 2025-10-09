use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::constants::TheString;
use crate::data::steam::achievement::Achievement;
use crate::data::steam::playtime::Playtime;
use crate::platforms::steam::data::{GameInfo, Payload_GetGlobalPercentages, Payload_GetPlayerAchievements, Payload_GetSchemaForGame};

/**
A single game, containing all of its achievements.
*/
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Game
{
	/// The list of achievements associated with this game.
	pub achievements: Vec<Achievement>,
	
	/// Flag denoting whether or not the game has any achievements.
	pub hasAchievements: bool,
	
	/// The app ID of the game.
	pub id: usize,
	
	/// The hash value used to retrieve the game's icon.
	pub iconHash: String,
	
	/// The timestamp of the last time the player played the game.
	pub lastPlayed: usize,
	
	/// The human-readable title of the game.
	pub name: String,
	
	/// The amount of time played across platforms and offline.
	pub playtime: Playtime,
}

impl PartialOrd for Game
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self.hasAchievements.partial_cmp(&other.hasAchievements)
		{
			None => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
			Some(c) => match c
			{
				Ordering::Equal => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
				Ordering::Greater => Some(Ordering::Less),
				Ordering::Less => Some(Ordering::Greater),
			},
		};
	}
}

impl From<GameInfo> for Game
{
	fn from(value: GameInfo) -> Self
	{
		let mut instance = Self::default();
		instance.update(&value);
		return instance;
	}
}

impl Game
{
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
	
	pub fn update(&mut self, info: &GameInfo)
	{
		self.id = info.appid;
		self.iconHash = info.img_icon_url.to_owned();
		self.lastPlayed = info.rtime_last_played;
		self.name = info.name.to_owned();
		self.playtime.update(info);
	}
	
	pub fn updateGlobalPercentages(&mut self, payload: &Payload_GetGlobalPercentages)
	{
		for gp in &payload.achievementpercentages.achievements
		{
			if let Some(achievement) = self.achievements.iter_mut()
				.find(|a| a.id == gp.name)
			{
				achievement.globalPercentage = match gp.percent.is_empty()
				{
					true => None,
					false => Some(gp.percent.to_owned()),
				};
			}
		}
	}
	
	pub fn updateAchievementsState(&mut self, payload: &Payload_GetPlayerAchievements)
	{
		for state in &payload.playerstats.achievements
		{
			match self.achievements.iter_mut()
				.find(|a| a.id == state.apiname)
			{
				None => self.achievements.push(state.clone().into()),
				Some(achievement) => achievement.updateState(&state),
			}
		}
	}
	
	pub fn updateAchievementsMetadata(&mut self, payload: &Payload_GetSchemaForGame)
	{
		self.hasAchievements = !&payload.game.availableGameStats.achievements.is_none();
		
		if let Some(achievementList) = &payload.game.availableGameStats.achievements
		{
			for metadata in achievementList
			{
				match self.achievements.iter_mut()
					.find(|a| a.id == metadata.name)
				{
					None => self.achievements.push(metadata.clone().into()),
					Some(achievement) => achievement.update(&metadata),
				}
			}
		}
	}
}
