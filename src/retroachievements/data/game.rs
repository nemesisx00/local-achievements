use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
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
	#[serde(default)]
	pub achievements: Vec<Achievement>,
	
	/// The number of achievements unlocked in Casual mode.
	#[serde(default)]
	pub awardedCasual: u64,
	
	/// The number of achievements unlocked in Hardcore mode.
	#[serde(default)]
	pub awardedHardcore: u64,
	
	/// The number of distinct users who have played the game.
	#[serde(default)]
	pub distinctPlayers: u64,
	
	/// The number of distinct users who have played the game in Casual mode.
	#[serde(default)]
	pub distinctPlayersCasual: u64,
	
	/// The number of distinct users who have played the game in Hardcore mode.
	#[serde(default)]
	pub distinctPlayersHardcore: u64,
	
	/// The highest award, if any, that the user has been awarded for this game.
	#[serde(default)]
	pub highestAward: Option<AwardKind>,
	
	/// The timestamp when the user was awarded their highest award.
	#[serde(default)]
	pub highestAwardedTimestamp: Option<String>,
	
	/// The relative path to the game's icon image on RetroAchievements.org.
	#[serde(default)]
	pub icon: String,
	
	/// The GameID of the game.
	pub id: u64,
	
	/// The timestamp of the most recently unlocked achievement.
	#[serde(default)]
	pub mostRecentTimestamp: Option<String>,
	
	/// The title of the game.
	#[serde(default)]
	pub name: String,
	
	/// The system on which the system is played.
	#[serde(default)]
	pub system: System,
	
	/// The total number of achievements in the set.
	#[serde(default)]
	pub total: u64,
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
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut game = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "awardedCasual")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					game.awardedCasual = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "awardedHardcore")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					game.awardedHardcore = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "distinctPlayers")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					game.distinctPlayers = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "distinctPlayersCasual")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					game.distinctPlayersCasual = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "distinctPlayersHardcore")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					game.distinctPlayersHardcore = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "highestAward")
		{
			if let Value::Number(number) = value
			{
				game.highestAward = Some(number.to_owned().into());
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "highestAwardedTimestamp")
		{
			if let Value::String(string) = value
			{
				if !string.is_empty()
				{
					game.highestAwardedTimestamp = Some(string.to_owned());
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "icon")
		{
			if let Value::String(string) = value
			{
				game.icon = string.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					game.id = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "system")
		{
			if let Value::Object(systemMap) = value
			{
				if let Some(system) = System::parseJsonMap(systemMap)
				{
					game.system = system;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "total")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					game.total = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "mostRecentTimestamp")
		{
			if let Value::String(string) = value
			{
				game.mostRecentTimestamp = match string.is_empty()
				{
					false => Some(string.to_owned()),
					true => None,
				};
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(string) = value
			{
				game.name = string.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "achievements")
		{
			if let Value::Array(achievements) = value
			{
				let mut parsedAchievements = vec![];
				for achievementValue in achievements
				{
					if let Value::Object(map) = achievementValue
					{
						if let Some(achievement) = Achievement::parseJsonMap(map)
						{
							parsedAchievements.push(achievement);
						}
					}
				}
				game.achievements = parsedAchievements;
			}
		}
		
		return match game.id
		{
			0 => None,
			_ => Some(game)
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

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(successful: bool) -> Map<String, Value>
	{
		let mut achievement1 = Map::new();
		achievement1.insert("id".into(), 1.into());
		achievement1.insert("name".into(), "First achievement".into());
		
		let mut achievement2 = Map::new();
		achievement2.insert("id".into(), "Fail achievement".into());
		
		let mut achievement3 = Map::new();
		achievement3.insert("id".into(), 3.into());
		achievement3.insert("name".into(), "Last achievement".into());
		
		let mut system = Map::new();
		system.insert("id".into(), 12.into());
		system.insert("name".into(), "The system".into());
		
		let mut map = Map::new();
		
		map.insert("achievements".into(), vec![
			achievement1,
			achievement2,
			achievement3,
		].into());
		
		map.insert("awardedCasual".into(), 0.into());
		map.insert("awardedHardcore".into(), 2.into());
		map.insert("distinctPlayers".into(), 4.into());
		map.insert("distinctPlayersCasual".into(), 1.into());
		map.insert("distinctPlayersHardcore".into(), 3.into());
		map.insert("highestAward".into(), (AwardKind::BeatenHardcore as u64).into());
		map.insert("highestAwardedTimestamp".into(), "The timestamp".into());
		map.insert("icon".into(), "The icon".into());
		
		if successful
		{
			map.insert("id".into(), 9.into());
		}
		
		map.insert("system".into(), system.into());
		map.insert("total".into(), 27.into());
		map.insert("mostRecentTimestamp".into(), Value::Null);
		map.insert("name".into(), "The name".into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let mut map = buildMap(false);
		let fail = Game::parseJsonMap(&map);
		assert_eq!(fail, None);
		
		map = buildMap(true);
		let success = Game::parseJsonMap(&map);
		assert_ne!(success, None);
		
		let game = success.unwrap();
		assert_eq!(game.achievements.len(), 2);
		assert_eq!(game.awardedCasual, 0);
		assert_eq!(game.awardedHardcore, 2);
		assert_eq!(game.distinctPlayers, 4);
		assert_eq!(game.distinctPlayersCasual, 1);
		assert_eq!(game.distinctPlayersHardcore, 3);
		assert_eq!(game.highestAward, Some(AwardKind::BeatenHardcore));
		assert_eq!(game.highestAwardedTimestamp, Some("The timestamp".to_string()));
		assert_eq!(game.icon, "The icon".to_string());
		assert_eq!(game.id, 9);
		assert_eq!(game.mostRecentTimestamp, None);
		assert_eq!(game.name, "The name".to_string());
		assert_eq!(game.system, System { id: 12, name: "The system".into() });
		
		assert!(game.achievements.iter().any(|a| a.id == 1));
		let firstAchievement = game.achievements.iter()
			.find(|a| a.id == 1)
			.unwrap();
		assert_eq!(firstAchievement.name, "First achievement".to_string());
		
		assert!(game.achievements.iter().any(|a| a.id == 3));
		let lastAchievement = game.achievements.iter()
			.find(|a| a.id == 3)
			.unwrap();
		assert_eq!(lastAchievement.name, "Last achievement".to_string());
	}
}
