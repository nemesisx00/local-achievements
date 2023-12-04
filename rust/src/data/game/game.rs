use std::collections::HashMap;
use ::serde::{Deserialize, Serialize};
use crate::platforms::Platform;
use crate::platforms::steam::{SteamAchievementData, SteamAchievementMetadata};
use super::retroachievements::{RetroAchievement, RetroAchievementsInfo, RetroMode};
use super::steam::{SteamInfo, SteamAchievement};

/**
A single game, containing all of its achievements.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Game
{
	/**
	Alternate or duplicate versions of this game which have their own distinct
	lists of achievements.
	
	Some games are re-released and treated as separate titles on the platform,
	in spite of being the exact same game. In rare cases, certain DLC can be
	released as a standalone title and then later combined with the main game
	into a third title, like so:
	
	- Deus Ex: Human Revolution
	- Deus Ex: Human Revolution - The Missing Link
	- Deus Ex: Human Revolution - Director's Cut
	*/
	pub duplicates: Option<Vec<Game>>,
	
	/// The title of this game.
	pub name: String,
	
	/// Information specific to RetroAchievements.org
	pub retroAchievements: Option<GamePlatform<RetroAchievementsInfo, RetroAchievement>>,
	
	/// Information specific to Steam
	pub steam: Option<GamePlatform<SteamInfo, SteamAchievement>>,
}

// Simple ordering based solely on the game's name.
impl PartialOrd for Game
{
	fn ge(&self, other: &Self) -> bool { return self.name.to_lowercase().ge(&other.name.to_lowercase()); }
	fn gt(&self, other: &Self) -> bool { return self.name.to_lowercase().gt(&other.name.to_lowercase()); }
	fn le(&self, other: &Self) -> bool { return self.name.to_lowercase().le(&other.name.to_lowercase()); }
	fn lt(&self, other: &Self) -> bool { return self.name.to_lowercase().lt(&other.name.to_lowercase()); }
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { return self.name.to_lowercase().partial_cmp(&other.name.to_lowercase()); }
}

impl Game
{
	pub fn getIds(&self) -> HashMap<Platform, String>
	{
		let mut ids = HashMap::new();
		
		if let Some(retro) = &self.retroAchievements
		{
			ids.insert(Platform::RetroAchievements, retro.info.id.to_owned());
		}
		
		if let Some(steam) = &self.steam
		{
			ids.insert(Platform::Steam, steam.info.id.to_string());
		}
		
		return ids;
	}
	
	/**
	Add a game to this game's list of duplicates.
	
	If the duplicate being added contains duplicates, those duplicates are also
	added to this game's list of duplicates.
	
	Note, however, that this method is not recursive. It only accumulates the
	dupe parameter's duplicates and any duplicates below that level are ignored.
	However, this should never be a problem as long as this method is always
	used to add duplicates to an instance of Game.
	*/
	pub fn addDuplicate(&mut self, dupe: Game)
	{
		if self.duplicates == None
		{
			self.duplicates = Some(vec![]);
		}
		
		if let Some(dupes) = self.duplicates.as_mut()
		{
			if let Some(otherDupes) = dupe.duplicates.clone()
			{
				for subdupe in otherDupes
				{
					let mut game = subdupe.clone();
					game.duplicates = None;
					
					if !dupes.contains(&game)
					{
						dupes.push(game.to_owned());
					}
				}
			}
			
			let mut game = dupe.clone();
			game.duplicates = None;
			
			if !dupes.contains(&game)
			{
				dupes.push(game.to_owned());
			}
		}
	}
}

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GamePlatform<I, A>
{
	/// Information specific to the Platform.
	pub info: I,
	/// The list of achievements associated with this game.
	pub achievements: Vec<A>,
}

impl GamePlatform<RetroAchievementsInfo, RetroAchievement>
{
	pub fn isGlobalPercentageMissing(&self) -> bool
	{
		return self.achievements.iter()
			.any(|a| a.globalPercentage.is_none());
	}
	
	/**
	Retrieve either the total accumulated points or the maximum possible points
	awarded for this game's achievements on RetroAchievements.org.
	
	Parameter | Type | Description
	---|---|---
	maximumPossible | Boolean | Whether (TRUE) or not (FALSE) to take unlock status into consideration when summing the points.
	mode | Mode | The mode which determines the amount of points per achievement.
	*/
	pub fn points(&self, mode: RetroMode, maximumPossible: bool) -> usize
	{
		let mut points = 0;
		for achievement in &self.achievements
		{
			if maximumPossible == true || achievement.mode.is_some_and(|m| m == mode)
			{
				if let Some(map) = &achievement.points
				{
					if let Some(value) = map.get(&mode)
					{
						points += *value;
					}
				}
			}
		}
		
		return points;
	}
	
	pub fn updateGlobalPercentages(&mut self, percentages: HashMap<String, f64>)
	{
		for (id, percentage) in percentages
		{
			if let Some(achievement) = self.achievements.iter_mut()
				.find(|a| a.id == id)
			{
				achievement.globalPercentage = Some(percentage);
			}
		}
	}
}

impl GamePlatform<SteamInfo, SteamAchievement>
{
	pub fn isGlobalPercentageMissing(&self) -> bool
	{
		return self.achievements.iter()
			.any(|a| a.globalPercentage.is_none());
	}
	
	pub fn updateAchievements(&mut self, achievements: Vec<SteamAchievementData>)
	{
		for achievement in achievements
		{
			match self.achievements.iter_mut()
				.find(|a| a.id == achievement.apiname)
			{
				Some(chievo) => chievo.update(achievement),
				None => self.achievements.push(SteamAchievement::from(achievement)),
			}
		}
	}
	
	pub fn updateAchievementMetadata(&mut self, achievements: Vec<SteamAchievementMetadata>)
	{
		for metadata in achievements
		{
			match self.achievements.iter_mut()
				.find(|a| a.id == metadata.name)
			{
				Some(chievo) => chievo.updateMetadata(metadata),
				None => self.achievements.push(SteamAchievement::from(metadata)),
			}
		}
	}
	
	pub fn updateGlobalPercentages(&mut self, percentages: HashMap<String, f64>)
	{
		for (id, percentage) in percentages
		{
			if let Some(achievement) = self.achievements.iter_mut()
				.find(|a| a.id == id)
			{
				achievement.globalPercentage = Some(percentage);
			}
		}
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	use std::collections::HashMap;
	
	fn setupRetroAchievement(name: &str, hcPoints: usize, scPoints: usize, mode: Option<RetroMode>) -> RetroAchievement
	{
		let mut points = HashMap::new();
		points.insert(RetroMode::Hardcore, hcPoints);
		points.insert(RetroMode::Softcore, scPoints);
		
		let achievement = RetroAchievement
		{
			description: String::default(),
			globalPercentage: None,
			id: String::default(),
			mode: mode,
			name: name.to_string(),
			points: Some(points),
			timestamp: match mode
			{
				Some(_) => Some(1),
				None => None,
			}
		};
		
		return achievement;
	}
	
    #[test]
    fn RetroPoints()
	{
		let mut instance = GamePlatform::<RetroAchievementsInfo, RetroAchievement>::default();
		instance.achievements.push(setupRetroAchievement("A1", 10, 5, Some(RetroMode::Softcore)));
		instance.achievements.push(setupRetroAchievement("A2", 20, 10, Some(RetroMode::Hardcore)));
		instance.achievements.push(setupRetroAchievement("A3", 15, 25, None));
		
		let hcExpected = 20;
		let hcResult = instance.points(RetroMode::Hardcore, false);
		assert_eq!(hcExpected, hcResult);
		
		let hcTotalExpected = 45;
		let hcTotalResult = instance.points(RetroMode::Hardcore, true);
		assert_eq!(hcTotalExpected, hcTotalResult);
		
		let scExpected = 5;
		let scResult = instance.points(RetroMode::Softcore, false);
		assert_eq!(scExpected, scResult);
		
		let scTotalExpected = 40;
		let scTotalResult = instance.points(RetroMode::Softcore, true);
		assert_eq!(scTotalExpected, scTotalResult);
	}
	
	#[test]
	fn GlobalPercentage()
	{
		let mut instance = GamePlatform::<RetroAchievementsInfo, RetroAchievement>::default();
		instance.achievements.push(setupRetroAchievement("A1", 0, 0, None));
		
		assert!(instance.isGlobalPercentageMissing());
		instance.achievements.iter_mut().for_each(|a| a.globalPercentage = Some(25.0));
		assert!(!instance.isGlobalPercentageMissing());
	}
}
