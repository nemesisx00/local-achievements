use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::constants::TheString;
use crate::steam::platform::{GameInfo, Payload_GetGlobalPercentages,
	Payload_GetPlayerAchievements, Payload_GetSchemaForGame};
use super::achievement::Achievement;
use super::playtime::Playtime;

/**
A single game, containing all of its achievements.
*/
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Game
{
	/// The list of achievements associated with this game.
	#[serde(default)]
	pub achievements: Vec<Achievement>,
	
	/// Flag denoting whether or not the game has any achievements.
	#[serde(default)]
	pub hasAchievements: bool,
	
	/// The app ID of the game.
	pub id: u64,
	
	/// The hash value used to retrieve the game's icon.
	#[serde(default)]
	pub iconHash: String,
	
	/// The timestamp of the last time the player played the game.
	#[serde(default)]
	pub lastPlayed: u64,
	
	/// Flag denoting whether or not the game's data has been loaded.
	#[serde(default)]
	pub loaded: bool,
	
	/// The human-readable title of the game.
	#[serde(default)]
	pub name: String,
	
	/// The amount of time played across platforms and offline.
	#[serde(default)]
	pub playtime: Playtime,
}

impl PartialOrd for Game
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self.loaded.partial_cmp(&other.loaded)
		{
			None => match self.hasAchievements.partial_cmp(&other.hasAchievements)
			{
				None => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
				Some(c) => match c
				{
					Ordering::Equal => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
					Ordering::Greater => Some(Ordering::Less),
					Ordering::Less => Some(Ordering::Greater),
				},
			},
			
			Some(c) => match c
			{
				Ordering::Equal => match self.hasAchievements.partial_cmp(&other.hasAchievements)
				{
					None => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
					Some(c) => match c
					{
						Ordering::Equal => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
						Ordering::Greater => Some(Ordering::Less),
						Ordering::Less => Some(Ordering::Greater),
					},
				},
				
				Ordering::Greater => Some(Ordering::Less),
				Ordering::Less => Some(Ordering::Greater),
			}
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
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut game = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(k, _)| k.as_str() == "achievements")
		{
			if let Value::Array(inner) = value
			{
				let mut parsedAchievements = vec![];
				for achievementValue in inner
				{
					if let Value::Object(achievementMap) = achievementValue
					{
						if let Some(achievement) = Achievement::parseJsonMap(achievementMap)
						{
							parsedAchievements.push(achievement);
						}
					}
				}
				
				game.achievements = parsedAchievements;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "hasAchievements")
		{
			if let Value::Bool(inner) = value
			{
				game.hasAchievements = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					game.id = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "iconHash")
		{
			if let Value::String(inner) = value
			{
				game.iconHash = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "lastPlayed")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					game.lastPlayed = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "loaded")
		{
			if let Value::Bool(inner) = value
			{
				game.loaded = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				game.name = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "playtime")
		{
			if let Value::Object(inner) = value
			{
				game.playtime = Playtime::parseJsonMap(inner);
			}
		}
		
		return match game.id
		{
			0 => None,
			_ => Some(game),
		};
	}
	
	pub fn percentUnlocked(&self) -> f64
	{
		return (self.achievements.iter()
				.filter(|a| a.unlocked())
				.count() as f64 / self.achievements.len() as f64)
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
		
		self.hasAchievements = !&payload.game.availableGameStats.achievements.is_none();
		self.loaded = true;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(success: bool) -> Map<String, Value>
	{
		let mut achievement1 = Map::new();
		achievement1.insert("id".into(), "First achievement".into());
		achievement1.insert("description".into(), "First description".into());
		
		let mut achievement2 = Map::new();
		achievement2.insert("description".into(), "Second description".into());
		
		let mut achievement3 = Map::new();
		achievement3.insert("id".into(), "Last achievement".into());
		achievement3.insert("description".into(), "Last description".into());
		
		let achievements = vec![
			achievement1,
			achievement2,
			achievement3,
		];
		
		let mut playtime = Map::new();
		playtime.insert("linux".into(), 20000.into());
		playtime.insert("mac".into(), 1.into());
		playtime.insert("offline".into(), 1000.into());
		playtime.insert("total".into(), 21101.into());
		playtime.insert("windows".into(), 100.into());
		
		let mut map = Map::new();
		map.insert("achievements".into(), achievements.into());
		map.insert("hasAchievements".into(), true.into());
		
		if success
		{
			map.insert("id".into(), 54321.into());
		}
		
		map.insert("iconHash".into(), "The icon hash".into());
		map.insert("lastPlayed".into(), 1029384756.into());
		map.insert("loaded".into(), true.into());
		map.insert("name".into(), "The name".into());
		map.insert("playtime".into(), playtime.into());
		
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
		assert_eq!(game.hasAchievements, true);
		assert_eq!(game.id, 54321);
		assert_eq!(game.iconHash, "The icon hash".to_string());
		assert_eq!(game.lastPlayed, 1029384756);
		assert_eq!(game.loaded, true);
		assert_eq!(game.name, "The name".to_string());
		assert_eq!(game.playtime, Playtime {
			linux: 20000,
			mac: 1,
			offline: 1000,
			total: 21101,
			windows: 100,
		});
		
		let firstId = "First achievement".to_string();
		assert!(game.achievements.iter().any(|t| t.id == firstId));
		let firstAchievement = game.achievements.iter()
			.find(|t| t.id == firstId)
			.unwrap();
		assert_eq!(firstAchievement.id, firstId);
		assert_eq!(firstAchievement.description, "First description".to_string());
		
		let lastId = "Last achievement".to_string();
		assert!(game.achievements.iter().any(|t| t.id == lastId));
		let lastAchievement = game.achievements.iter()
			.find(|t| t.id == lastId)
			.unwrap();
		assert_eq!(lastAchievement.id, lastId);
		assert_eq!(lastAchievement.description, "Last description".to_string());
	}
}
