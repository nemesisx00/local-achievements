use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::constants::TheString;
use crate::retroachievements::platform::{GameMetadata, Payload_GetGameInfo};
use super::makeRelative;
use super::achievement::Achievement;
use super::kind::AwardKind;
use super::mode::AchievementMode;
use super::system::System;

/**
The 
*/
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Game
{
	/// The list of achievements in the set.
	pub achievements: Vec<Achievement>,
	
	/// The number of achievements unlocked in Casual mode.
	pub awardedCasual: usize,
	
	/// The number of achievements unlocked in Hardcore mode.
	pub awardedHardcore: usize,
	
	/// The number of distinct users who have played the game.
	pub distinctPlayers: usize,
	
	/// The number of distinct users who have played the game in Casual mode.
	pub distinctPlayersCasual: usize,
	
	/// The number of distinct users who have played the game in Hardcore mode.
	pub distinctPlayersHardcore: usize,
	
	/// The highest award, if any, that the user has been awarded for this game.
	pub highestAward: Option<AwardKind>,
	
	/// The timestamp when the user was awarded their highest award.
	pub highestAwardedTimestamp: Option<String>,
	
	/// The relative path to the game's icon image on RetroAchievements.org.
	pub icon: String,
	
	/// The GameID of the game.
	pub id: usize,
	
	/// The system on which the system is played.
	pub system: System,
	
	/// The total number of achievements in the set.
	pub total: usize,
	
	/// The timestamp of the most recently unlocked achievement.
	pub mostRecentTimestamp: Option<String>,
	
	/// The title of the game.
	pub name: String,
}

impl From<GameMetadata> for Game
{
	fn from(value: GameMetadata) -> Self
	{
		let mut instance = Self::default();
		instance.update(&value);
		return instance;
	}
}

impl From<Payload_GetGameInfo> for Game
{
	fn from(value: Payload_GetGameInfo) -> Self
	{
		let mut instance = Self::default();
		instance.updateDetailed(&value);
		return instance;
	}
}

impl PartialOrd for Game
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase())
		{
			None => self.system.partial_cmp(&other.system),
			
			Some(o) => match o
			{
				Ordering::Equal => self.system.partial_cmp(&other.system),
				_ => Some(o),
			},
		};
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
	
	pub fn percentUnlocked(&self, mode: AchievementMode) -> f64
	{
		return (match mode
		{
			AchievementMode::Casual => self.awardedCasual,
			AchievementMode::Hardcore => self.awardedHardcore,
		} as f64
			/ self.total as f64)
		* 100.0;
	}
	
	pub fn update(&mut self, game: &GameMetadata)
	{
		self.awardedCasual = game.NumAwarded;
		self.awardedHardcore = game.NumAwardedHardcore;
		self.highestAwardedTimestamp = game.HighestAwardDate.to_owned();
		
		match &game.HighestAwardKind
		{
			None => self.highestAward = None,
			Some(hak) => self.highestAward = AwardKind::parse(hak),
		}
		
		self.icon = makeRelative(&game.ImageIcon);
		self.id = game.GameID;
		self.total = game.MaxPossible;
		self.mostRecentTimestamp = game.MostRecentAwardedDate.to_owned();
		self.name = game.Title.to_owned();
		self.system = game.to_owned().into();
	}
	
	pub fn updateDetailed(&mut self, game: &Payload_GetGameInfo)
	{
		self.achievements = game.Achievements.iter()
			.map(|(_, a)| a.to_owned().into())
			.collect();
		
		self.awardedCasual = game.NumAwardedToUser;
		self.awardedHardcore = game.NumAwardedToUserHardcore;
		self.distinctPlayers = game.NumDistinctPlayers;
		self.distinctPlayersCasual = game.NumDistinctPlayersCasual;
		self.distinctPlayersHardcore = game.NumDistinctPlayersHardcore;
		
		self.highestAward = match &game.HighestAwardKind
		{
			None => None,
			Some(hak) => AwardKind::parse(hak),
		};
		
		self.highestAwardedTimestamp = game.HighestAwardDate.to_owned();
		self.icon = makeRelative(&game.ImageIcon);
		self.id = game.ID;
		self.name = game.Title.to_owned();
		self.system = game.to_owned().into();
		self.total = game.NumAchievements;
	}
}
