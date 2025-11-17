use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::rpcs3::platform::data::conf::TrophyConf;

use super::trophy::Trophy;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Game
{
	#[serde(default)]
	pub detail: String,
	
	#[serde(default)]
	pub name: String,
	
	pub npCommId: String,
	
	#[serde(default)]
	pub parentalLevel: i64,
	
	#[serde(default)]
	pub trophies: Vec<Trophy>,
	
	#[serde(default)]
	pub trophySetVersion: String,
}

impl From<TrophyConf> for Game
{
	fn from(value: TrophyConf) -> Self
	{
		return Self
		{
			detail: value.titleDetail.to_owned(),
			name: value.titleName.to_owned(),
			npCommId: value.npcommid.to_owned(),
			parentalLevel: value.parentalLevel.value as i64,
			trophies: value.trophies.iter()
				.cloned()
				.map(|t| t.into())
				.collect(),
			trophySetVersion: value.trophysetVersion.to_owned(),
		};
	}
}

impl PartialOrd for Game
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return self.name.partial_cmp(&other.name);
	}
}

impl Game
{
	const MaxPoints_Disc: u64 = 1230;
	#[allow(unused)]
	const MaxPoints_Psn: u64 = 315;
	#[allow(unused)]
	const MaxPoints_Dlc: u64 = 200;
	
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut game = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "detail")
		{
			if let Value::String(inner) = value
			{
				game.detail = inner.to_owned();
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
			.find(|(key, _)| key.as_str() == "npCommId")
		{
			if let Value::String(inner) = value
			{
				game.npCommId = inner.to_owned();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "parentalLevel")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_i64()
				{
					game.parentalLevel = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(k, _)| k.as_str() == "trophies")
		{
			if let Value::Array(inner) = value
			{
				let mut parsedTrophies = vec![];
				for trophyValue in inner
				{
					if let Value::Object(trophyMap) = trophyValue
					{
						if let Some(trophy) = Trophy::parseJsonMap(trophyMap)
						{
							parsedTrophies.push(trophy);
						}
					}
				}
				
				game.trophies = parsedTrophies;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "trophySetVersion")
		{
			if let Value::String(inner) = value
			{
				game.trophySetVersion = inner.to_owned();
			}
		}
		
		return match game.npCommId.is_empty()
		{
			false => Some(game),
			true => None,
		};
	}
	
	pub fn percentUnlocked(&self) -> f32
	{
		return (self.trophies.iter()
					.filter(|t| t.unlocked == true)
					.count() as f32
				/ self.trophies.len() as f32)
			* 100f32;
	}
	
	pub fn points(&self) -> u64
	{
		let points = self.trophies.iter()
			.filter(|t| t.unlocked)
			.fold(0, |acc, t| acc + t.grade.points());
		
		return match points > Self::MaxPoints_Disc
		{
			false => points,
			true => Self::MaxPoints_Disc,
		};
	}
	
	pub fn update(&mut self, game: &Self)
	{
		self.detail = game.detail.to_owned();
		self.name = game.name.to_owned();
		self.npCommId = game.npCommId.to_owned();
		self.parentalLevel = game.parentalLevel.to_owned();
		self.trophySetVersion = game.trophySetVersion.to_owned();
		
		for trophy in self.trophies.iter_mut()
		{
			if let Some(other) = game.trophies.iter()
				.find(|t| t.id == trophy.id)
			{
				trophy.update(&other);
			}
		}
		
		let this = self.clone();
		for trophy in game.trophies.iter()
			.filter(|t| !this.trophies.contains(t))
		{
			self.trophies.push(trophy.to_owned());
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(success: bool) -> Map<String, Value>
	{
		let mut trophy1 = Map::new();
		trophy1.insert("id".into(), 1.into());
		trophy1.insert("name".into(), "First trophy".into());
		
		let mut trophy2 = Map::new();
		trophy2.insert("name".into(), "Second trophy".into());
		
		let mut trophy3 = Map::new();
		trophy3.insert("id".into(), 3.into());
		trophy3.insert("name".into(), "Last trophy".into());
		
		let trophies = vec![
			trophy1,
			trophy2,
			trophy3,
		];
		
		let mut map = Map::new();
		map.insert("detail".into(), "The detail".into());
		map.insert("name".into(), "The name".into());
		
		if success
		{
			map.insert("npCommId".into(), "abc0001122_333".into());
		}
		
		map.insert("parentalLevel".into(), 1.into());
		map.insert("trophies".into(), trophies.into());
		map.insert("trophySetVersion".into(), "1.0".into());
		
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
		assert_eq!(game.detail, "The detail".to_string());
		assert_eq!(game.name, "The name".to_string());
		assert_eq!(game.npCommId, "abc0001122_333".to_string());
		assert_eq!(game.parentalLevel, 1);
		assert_eq!(game.trophies.len(), 2);
		assert_eq!(game.trophySetVersion, "1.0".to_string());
		
		assert!(game.trophies.iter().any(|t| t.id == 1));
		let firstTrophy = game.trophies.iter()
			.find(|t| t.id == 1)
			.unwrap();
		assert_eq!(firstTrophy.name, "First trophy".to_string());
		
		assert!(game.trophies.iter().any(|t| t.id == 3));
		let lastTrophy = game.trophies.iter()
			.find(|t| t.id == 3)
			.unwrap();
		assert_eq!(lastTrophy.name, "Last trophy".to_string());
	}
}
