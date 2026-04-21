use std::cmp::Ordering;
use data::constants::TheString;
use data::filter::{FilterCriteria, Filterable};
use data::format::truncateF32;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::api::{AppInfo, GameInfo, Payload_GetGlobalPercentages,
	Payload_GetPlayerAchievements, Payload_GetSchemaForGame};
use super::achievement::SteamAchievement;
use super::playtime::Playtime;

/**
A single game, containing all of its achievements.
*/
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Game
{
	/// The list of achievements associated with this game.
	#[serde(default)]
	pub achievements: Vec<SteamAchievement>,
	
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

impl Filterable<SteamAchievement> for Game
{
	fn filter(&self, filter: impl Into<FilterCriteria>) -> Vec<SteamAchievement>
	{
		let filter = filter.into();
		
		let caseSensitive = filter.caseSensitive;
		let locked = filter.locked;
		let nameOnly = filter.nameOnly;
		
		let search = match caseSensitive
		{
			false => filter.text.to_lowercase(),
			true => filter.text.clone(),
		};
		
		let mut achievements = self.achievements.iter()
			.filter(|a| match locked
			{
				false => true,
				true => !a.unlocked(),
			})
			.filter(|a| match caseSensitive
			{
				false => match nameOnly
				{
					false => a.name.to_lowercase().contains(&search)
						|| a.description.to_lowercase().contains(&search),
					true => a.name.to_lowercase().contains(&search),
				},
				
				true => match nameOnly
				{
					false => a.name.contains(&search)
						|| a.description.contains(&search),
					true => a.name.contains(&search),
				}
			})
			.cloned()
			.collect::<Vec<_>>();
		
		achievements.sort();
		
		return achievements;
	}
}

impl From<AppInfo> for Game
{
	fn from(value: AppInfo) -> Self
	{
		let mut instance = Self::default();
		instance.updateShared(&value);
		return instance;
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

impl PartialOrd for Game
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self.loaded.partial_cmp(&other.loaded)
		{
			None => match self.hasAchievements.partial_cmp(&other.hasAchievements)
			{
				None => match self.lastPlayed.partial_cmp(&other.lastPlayed)
				{
					None => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
					Some(c) => match c
					{
						Ordering::Equal => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
						Ordering::Greater => Some(Ordering::Less),
						Ordering::Less => Some(Ordering::Greater),
					}
				},
				
				Some(c) => match c
				{
					Ordering::Equal => match self.lastPlayed.partial_cmp(&other.lastPlayed)
					{
						None => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
						Some(c) => match c
						{
							Ordering::Equal => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
							Ordering::Greater => Some(Ordering::Less),
							Ordering::Less => Some(Ordering::Greater),
						}
					},
					
					Ordering::Greater => Some(Ordering::Less),
					Ordering::Less => Some(Ordering::Greater),
				},
			},
			
			Some(c) => match c
			{
				Ordering::Equal => match self.hasAchievements.partial_cmp(&other.hasAchievements)
				{
					None => match self.lastPlayed.partial_cmp(&other.lastPlayed)
					{
						None => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
						Some(c) => match c
						{
							Ordering::Equal => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
							Ordering::Greater => Some(Ordering::Less),
							Ordering::Less => Some(Ordering::Greater),
						}
					}
					
					Some(c) => match c
					{
						Ordering::Equal => match self.lastPlayed.partial_cmp(&other.lastPlayed)
						{
							None => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
							Some(c) => match c
							{
								Ordering::Equal => self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase()),
								Ordering::Greater => Some(Ordering::Less),
								Ordering::Less => Some(Ordering::Greater),
							}
						},
						
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
						if let Some(achievement) = SteamAchievement::parseJsonMap(achievementMap)
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
				game.hasAchievements = inner.clone();
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
				game.iconHash = inner.clone();
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
				game.loaded = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				game.name = inner.clone();
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
	
	pub fn percentUnlocked(&self) -> f32
	{
		return truncateF32(
			(
					self.achievements.iter()
						.filter(|a| a.unlocked())
						.count() as f32
					/ self.achievements.len() as f32
				)
				* 100.0,
			2
		);
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
	
	pub fn update(&mut self, info: &GameInfo)
	{
		self.id = info.appid;
		self.iconHash = info.img_icon_url.clone();
		self.lastPlayed = info.rtime_last_played;
		self.name = info.name.clone();
		self.playtime.update(info);
	}
	
	pub fn updateShared(&mut self, info: &AppInfo)
	{
		self.id = info.appid;
		
		if let Some(hash) = &info.img_icon_hash
		{
			self.iconHash = hash.clone();
		}
		
		self.lastPlayed = info.rt_last_played;
		self.name = info.name.clone();
		self.playtime.updateShared(info);
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
					false => Some(gp.percent.clone()),
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
