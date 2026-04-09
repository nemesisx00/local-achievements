use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::egs::platform::data::private::AchievementSummary;
use crate::util::truncateF32;
use super::achievements::achievement::EgsAchievement;
use super::set::AchievementSet;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct EgsGame
{
	#[serde(default)]
	pub achievements: Vec<EgsAchievement>,
	
	#[serde(default)]
	pub achievementsCount: u64,
	
	#[serde(default)]
	pub achievementSets: Vec<AchievementSet>,
	
	#[serde(default)]
	pub achievementsUnlocked: u64,
	
	#[serde(default)]
	pub currentXp: u64,
	
	#[serde(default)]
	pub maxXp: u64,
	
	#[serde(default)]
	pub name: String,
	
	#[serde(default)]
	/**
	To work around Rust's decision to not support the `Eq` trait on floating
	point types, the value is stored as an unsigned integer. Divide by 10 to
	get the actual f64 value.
	*/
	pub platinumRarity: u64,
	
	#[serde(default)]
	pub productId: String,
	
	pub sandboxId: String,
}

impl From<AchievementSummary> for EgsGame
{
	fn from(value: AchievementSummary) -> Self
	{
		return Self
		{
			achievementsCount: value.productAchievements.totalAchievements,
			achievementsUnlocked: value.totalUnlocked,
			currentXp: value.totalXP,
			name: value.product.name.clone(),
			maxXp: value.productAchievements.totalProductXP,
			sandboxId: value.sandboxId.clone(),
			..Default::default()
		};
	}
}

impl PartialOrd for EgsGame
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return self.name.partial_cmp(&other.name);
	}
}

impl EgsGame
{
	pub fn filterAchievements(&self, search: impl Into<String>) -> Vec<EgsAchievement>
	{
		let search = search.into().to_lowercase();
		let mut achievements = self.achievements.iter()
			.filter(|a| a.locked.filterForText(&search)
				|| a.unlocked.filterForText(&search))
			.cloned()
			.collect::<Vec<_>>();
		achievements.sort();
		
		return achievements;
	}
	
	pub fn parseJsonMap(map: &Map<String, Value>) -> Option<Self>
	{
		let mut game = Self::default();
		
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
						if let Some(achievement) = EgsAchievement::parseJsonMap(map)
						{
							parsedAchievements.push(achievement);
						}
					}
				}
				game.achievements = parsedAchievements;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "achievementSets")
		{
			if let Value::Array(inner) = value
			{
				let mut parsedSets = vec![];
				for setValue in inner
				{
					if let Value::Object(innerMap) = setValue
					{
						if let Some(achievement) = AchievementSet::parseJsonMap(innerMap)
						{
							parsedSets.push(achievement);
						}
					}
				}
				game.achievementSets = parsedSets;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "achievementsUnlocked")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					game.achievementsUnlocked = number;
				}
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
			.find(|(key, _)| key.as_str() == "platinumRarity")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					game.platinumRarity = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "productId")
		{
			if let Value::String(inner) = value
			{
				game.productId = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "sandboxId")
		{
			if let Value::String(inner) = value
			{
				game.sandboxId = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "currentXp")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					game.currentXp = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "maxXp")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					game.maxXp = number;
				}
			}
		}
		
		return match game.sandboxId.is_empty()
		{
			false => Some(game),
			true => None,
		};
	}
	
	pub fn percentUnlocked(&self) -> f32
	{
		return truncateF32(
			match self.achievementsCount > 0
			{
				false => 0.0,
				true => (self.achievementsUnlocked as f32
						/ self.achievementsCount as f32)
					* 100.0
			},
			2
		);
	}
	
	pub fn updateSummary(&mut self, summary: AchievementSummary)
	{
		self.achievementsCount = summary.productAchievements.totalAchievements;
		self.achievementsUnlocked = summary.totalUnlocked;
		self.currentXp = summary.totalXP;
		self.name = summary.product.name.clone();
		self.maxXp = summary.productAchievements.totalProductXP;
		self.sandboxId = summary.sandboxId.clone();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap(successful: bool) -> Map<String, Value>
	{
		let mut firstUnlocked = Map::new();
		firstUnlocked.insert("name".into(), "First achievement".into());
		
		let mut secondUnlocked = Map::new();
		secondUnlocked.insert("name".into(), "Second achievement".into());
		
		let mut thirdUnlocked = Map::new();
		thirdUnlocked.insert("name".into(), "Last achievement".into());
		
		let mut achievement1 = Map::new();
		achievement1.insert("id".into(), "1".into());
		achievement1.insert("unlocked".into(), firstUnlocked.into());
		
		let mut achievement2 = Map::new();
		achievement2.insert("unlocked".into(), secondUnlocked.into());
		
		let mut achievement3 = Map::new();
		achievement3.insert("id".into(), "3".into());
		achievement3.insert("unlocked".into(), thirdUnlocked.into());
		
		let mut set1 = Map::new();
		set1.insert("id".into(), "only set".into());
		set1.insert("totalAchievements".into(), 5.into());
		
		let mut set2 = Map::new();
		set2.insert("totalAchievements".into(), 2.into());
		
		let mut map = Map::new();
		
		map.insert("achievements".into(), vec![
			achievement1,
			achievement2,
			achievement3,
		].into());
		
		map.insert("achievementSets".into(), vec![
			set1,
			set2,
		].into());
		
		map.insert("achievementsUnlocked".into(), 3.into());
		map.insert("name".into(), "The name".into());
		map.insert("productId".into(), "The product id".into());
		
		if successful
		{
			map.insert("sandboxId".into(), "The sandbox id".into());
		}
		
		map.insert("currentXp".into(), 15.into());
		map.insert("maxXp".into(), 25.into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let mut map = buildMap(false);
		let fail = EgsGame::parseJsonMap(&map);
		assert_eq!(fail, None);
		
		map = buildMap(true);
		let success = EgsGame::parseJsonMap(&map);
		assert_ne!(success, None);
		
		let game = success.unwrap();
		assert_eq!(game.achievements.len(), 2);
		assert_eq!(game.achievementSets.len(), 1);
		assert_eq!(game.achievementsUnlocked, 3);
		assert_eq!(game.currentXp, 15);
		assert_eq!(game.maxXp, 25);
		assert_eq!(&game.name, "The name");
		assert_eq!(&game.productId, "The product id");
		assert_eq!(&game.sandboxId, "The sandbox id");
		
		assert!(game.achievements.iter().any(|a| &a.id == "1"));
		let firstAchievement = game.achievements.iter()
			.find(|a| &a.id == "1")
			.unwrap();
		assert_eq!(&firstAchievement.unlocked.name, "First achievement");
		
		assert!(game.achievements.iter().any(|a| &a.id == "3"));
		let lastAchievement = game.achievements.iter()
			.find(|a| &a.id == "3")
			.unwrap();
		assert_eq!(&lastAchievement.unlocked.name, "Last achievement");
		
		let set = game.achievementSets.first().unwrap();
		assert_eq!(&set.id, "only set");
		assert_eq!(set.totalAchievements, 5);
	}
}
