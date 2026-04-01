use std::cmp::Ordering;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::constants::TheString;
use crate::gog::data::achievement::GogAchievement;
use crate::gog::platform::data::listing::Product;
use crate::util::truncateF32;

/**
The 
*/
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Ord, Serialize)]
pub struct Game
{
	/// The list of achievements in the set.
	#[serde(default)]
	pub achievements: Vec<GogAchievement>,
	
	/// 
	#[serde(default)]
	pub flags: GameFlags,
	
	/// 
	#[serde(default)]
	pub hasAchievements: Option<bool>,
	
	/// 
	#[serde(default)]
	pub id: u64,
	
	/// The title of the game.
	#[serde(default)]
	pub name: String,
	
	/// 
	#[serde(default)]
	pub rating: u64,
	
	/// 
	#[serde(default)]
	pub releaseDate: Option<i64>,
	
	/// 
	#[serde(default)]
	pub slug: String,
	
	/// 
	#[serde(default)]
	pub storePageUrl: String,
	
	/// 
	#[serde(default)]
	pub tags: Vec<String>,
}

impl From<Product> for Game
{
	fn from(value: Product) -> Self
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

impl Game
{
	const ReleaseDateFormat: &str = "%F %T%.6f%:z";
	
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut game = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "flags")
		{
			if let Value::Object(obj) = value
			{
				game.flags = GameFlags::parseJsonMap(obj);
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
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(string) = value
			{
				game.name = string.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "rating")
		{
			if let Value::Number(number) = value
			{
				if let Some(uint) = number.as_u64()
				{
					game.rating = uint;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "releaseDate")
		{
			if let Value::Number(number) = value
			{
				if let Some(int) = number.as_i64()
				{
					game.releaseDate = Some(int);
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "slug")
		{
			if let Value::String(string) = value
			{
				game.slug = string.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "storePageUrl")
		{
			if let Value::String(string) = value
			{
				game.storePageUrl = string.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "tags")
		{
			if let Value::Array(values) = value
			{
				let mut tags = vec![];
				for tag in values
				{
					if let Value::String(string) = tag
					{
						tags.push(string.clone());
					}
				}
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
						if let Some(achievement) = GogAchievement::parseJsonMap(map)
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
	
	pub fn filterAchievements(&self, search: impl Into<String>) -> Vec<GogAchievement>
	{
		let search = search.into().to_lowercase();
		let mut achievements = self.achievements.iter()
			.filter(|a| a.name.to_lowercase().contains(&search)
				|| a.description.to_lowercase().contains(&search))
			.cloned()
			.collect::<Vec<_>>();
		achievements.sort();
		
		return achievements;
	}
	
	pub fn percentUnlocked(&self) -> f32
	{
		return match self.achievements.is_empty()
		{
			false => truncateF32(
				(self.achievements.iter()
							.filter(|a| a.dateUnlocked.is_some())
							.count() as f32
						/ self.achievements.len() as f32)
					* 100.0,
				2
			),
			
			true => 0.0,
		};
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
	
	pub fn update(&mut self, game: &Product)
	{
		self.flags = GameFlags::from(game);
		self.id = game.id;
		self.name = game.title.clone();
		self.rating = game.rating;
		
		self.releaseDate = match &game.releaseDate
		{
			None => None,
			Some(value) => match DateTime::parse_from_str(
					format!("{}{}", value.date, value.timezone).as_str(),
					Self::ReleaseDateFormat
				)
				{
					Err(_) => None,
					Ok(date) => Some(date.timestamp()),
				},
		};
		
		self.slug = game.slug.clone();
		self.storePageUrl = game.url.clone();
		
		//TODO: Translate the tag id to the tag name
		self.tags = game.tags.clone();
	}
	
	pub fn updateAchievements(&mut self, achievements: impl Into<Vec<GogAchievement>>)
	{
		self.achievements = achievements.into();
		self.hasAchievements = Some(self.achievements.len() > 0);
	}
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct GameFlags
{
	available: bool,
	availableInAccount: bool,
	baseProductMissing: bool,
	comingSoon: bool,
	galaxyCompatible: bool,
	game: bool,
	hidden: bool,
	hidingDisabled: bool,
	inDevelopment: bool,
	movie: bool,
	new: bool,
}

impl From<&Product> for GameFlags
{
	fn from(value: &Product) -> Self
	{
		return Self
		{
			available: value.availability.isAvailable,
			availableInAccount: value.availability.isAvailableInAccount,
			baseProductMissing: value.isBaseProductMissing,
			comingSoon: value.isComingSoon,
			galaxyCompatible: value.isGalaxyCompatible,
			game: value.isGame,
			hidden: value.isHidden,
			hidingDisabled: value.isHidingDisabled,
			inDevelopment: value.isInDevelopment,
			movie: value.isMovie,
			new: value.isNew,
		};
	}
}

impl GameFlags
{
	pub fn parseJsonMap(map: &Map<String, Value>) -> Self
	{
		let mut game = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "available")
		{
			if let Value::Bool(inner) = value
			{
				game.available = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "availableInAccount")
		{
			if let Value::Bool(inner) = value
			{
				game.availableInAccount = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "baseProductMissing")
		{
			if let Value::Bool(inner) = value
			{
				game.baseProductMissing = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "comingSoon")
		{
			if let Value::Bool(inner) = value
			{
				game.comingSoon = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "galaxyCompatible")
		{
			if let Value::Bool(inner) = value
			{
				game.galaxyCompatible = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "game")
		{
			if let Value::Bool(inner) = value
			{
				game.game = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "hidden")
		{
			if let Value::Bool(inner) = value
			{
				game.hidden = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "hidingDisabled")
		{
			if let Value::Bool(inner) = value
			{
				game.hidingDisabled = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "inDevelopment")
		{
			if let Value::Bool(inner) = value
			{
				game.inDevelopment = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "movie")
		{
			if let Value::Bool(inner) = value
			{
				game.movie = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "new")
		{
			if let Value::Bool(inner) = value
			{
				game.new = *inner;
			}
		}
		
		return game;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(successful: bool) -> Map<String, Value>
	{
		let mut achievement1 = Map::new();
		achievement1.insert("id".into(), "1".into());
		achievement1.insert("name".into(), "First achievement".into());
		
		let mut achievement2 = Map::new();
		achievement2.insert("id".into(), "Fail achievement".into());
		
		let mut achievement3 = Map::new();
		achievement3.insert("id".into(), "3".into());
		achievement3.insert("name".into(), "Last achievement".into());
		
		let mut gameFlags = Map::new();
		gameFlags.insert("game".into(), true.into());
		
		let mut map = Map::new();
		
		map.insert("achievements".into(), vec![
			achievement1,
			achievement2,
			achievement3,
		].into());
		
		map.insert("awardedCasual".into(), gameFlags.into());
		
		if successful
		{
			map.insert("id".into(), 9.into());
		}
		
		map.insert("name".into(), "The name".into());
		map.insert("rating".into(), 4.into());
		map.insert("releaseDate".into(), 12345.into());
		map.insert("slug".into(), "The slug".into());
		map.insert("storePageUrl".into(), "The store page url".into());
		map.insert("tags".into(), vec!["tag1".to_string(), "tag2".to_string()].into());
		
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
		assert_eq!(game.id, 9);
		assert_eq!(&game.name, "The name");
		assert_eq!(game.rating, 4);
		assert_eq!(game.releaseDate, Some(12345));
		assert_eq!(&game.slug, "The slug");
		assert_eq!(&game.storePageUrl, "The store page url");
		assert_eq!(game.tags, vec!["tag1".to_string(), "tag2".to_string()]);
		
		assert!(game.achievements.iter().any(|a| &a.id == "1"));
		let firstAchievement = game.achievements.iter()
			.find(|a| &a.id == "1")
			.unwrap();
		assert_eq!(firstAchievement.name, "First achievement".to_string());
		
		assert!(game.achievements.iter().any(|a| &a.id == "3"));
		let lastAchievement = game.achievements.iter()
			.find(|a| &a.id == "3")
			.unwrap();
		assert_eq!(lastAchievement.name, "Last achievement".to_string());
	}
}
