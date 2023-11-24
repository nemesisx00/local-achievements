#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use ::serde::{Deserialize, Serialize};
use crate::platforms::Platform;
use crate::platforms::steam::SteamGame;
use super::super::achievement::{Achievement, Mode};
use super::retroachievements::RetroAchievementsInfo;
use super::steam::SteamInfo;

/**
A single game, containing all of its achievements.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Game
{
	/// The list of achievements associated with this game.
	pub achievements: Vec<Achievement>,
	
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
	pub retroAchievements: Option<RetroAchievementsInfo>,
	
	/// Information specific to Steam
	pub steam: Option<SteamInfo>,
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
	pub fn new(info: SteamGame) -> Self
	{
		let mut instance = Self::default();
		instance.setSteamInfo(info);
		return instance;
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
	
	/**
	Check if any of this game's achievements are missing a global percentage
	value for a given platform.
	*/
	pub fn isGlobalPercentageMissing(&self, platform: Platform) -> bool
	{
		return self.achievements.iter()
			.any(|a| !a.hasGlobalPercentage(platform));
	}
	
	/**
	Retrieve either the total accumulated points or the maximum possible points
	awarded for this game's achievements on RetroAchievements.org.
	
	Parameter | Type | Description
	---|---|---
	maximumPossible | Boolean | Whether (TRUE) or not (FALSE) to take unlock status into consideration when summing the points.
	mode | Mode | The mode which determines the amount of points per achievement.
	*/
	pub fn retroPoints(&self, mode: Mode, maximumPossible: bool) -> usize
	{
		let mut points = 0;
		for achievement in &self.achievements
		{
			if let Some(info) = achievement.platforms.iter().find(|a| a.platform == Platform::RetroAchievements)
			{
				if maximumPossible == true || info.mode.is_some_and(|m| m == mode)
				{
					if let Some(map) = &info.points
					{
						if let Some(value) = map.get(&mode)
						{
							points += *value;
						}
					}
				}
			}
		}
		
		return points;
	}
	
	/**
	Create or update this game's SteamInfo based on the information returned
	from the Steam Web API.
	*/
	pub fn setSteamInfo(&mut self, info: SteamGame)
	{
		self.name = info.name.to_owned();
		match self.steam.as_mut()
		{
			Some(steam) => steam.update(info),
			None => self.steam = Some(SteamInfo::new(info)),
		}
	}
	
	pub fn updateGlobalPercentages(&mut self, platform: Platform, percentages: HashMap<String, f64>)
	{
		for (id, percentage) in percentages
		{
			if let Some(achievement) = self.achievements.iter_mut()
				.find(|a| match a.platforms.iter().find(|pi| pi.platform == platform)
				{
					Some(pi) => pi.id == id,
					None => false,
				})
			{
				achievement.updateGlobalPercentage(platform, percentage);
			}
		}
	}
	
	pub fn updateAchievementMetadata(&mut self, achievements: Vec<Achievement>)
	{
		for achievement in achievements
		{
			let ids = achievement.getIds();
			match self.achievements.iter_mut()
				.find(|a| a.platforms.iter().find(|p| ids.contains(&p.id)).is_some())
			{
				Some(chievo) => chievo.update(achievement),
				None => self.achievements.push(achievement.to_owned()),
			}
		}
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	use std::collections::HashMap;
	use crate::data::PlatformInfo;
	use crate::data::achievement::Mode;
	
	fn setupAchievement(name: &str, platform: Platform, hcPoints: usize, scPoints: usize, mode: Option<Mode>) -> Achievement
	{
		let mut points = HashMap::new();
		points.insert(Mode::Hardcore, hcPoints);
		points.insert(Mode::Softcore, scPoints);
		
		let achievement = Achievement::new(PlatformInfo
			{
				description: String::default(),
				globalPercentage: None,
				id: String::default(),
				icon: None,
				iconLocked: None,
				mode: mode,
				name: name.to_string(),
				platform: platform,
				points: Some(points),
				timestamp: match mode
				{
					Some(_) => Some(1),
					None => None,
				}
			});
		
		return achievement;
	}
	
    #[test]
    fn RetroPoints()
	{
		let mut instance = Game::default();
		instance.achievements.push(setupAchievement("A1", Platform::RetroAchievements, 10, 5, Some(Mode::Softcore)));
		instance.achievements.push(setupAchievement("A2", Platform::RetroAchievements, 20, 10, Some(Mode::Hardcore)));
		instance.achievements.push(setupAchievement("A3", Platform::RetroAchievements, 15, 25, None));
		
		let hcExpected = 20;
		let hcResult = instance.retroPoints(Mode::Hardcore, false);
		assert_eq!(hcExpected, hcResult);
		
		let hcTotalExpected = 45;
		let hcTotalResult = instance.retroPoints(Mode::Hardcore, true);
		assert_eq!(hcTotalExpected, hcTotalResult);
		
		let scExpected = 5;
		let scResult = instance.retroPoints(Mode::Softcore, false);
		assert_eq!(scExpected, scResult);
		
		let scTotalExpected = 40;
		let scTotalResult = instance.retroPoints(Mode::Softcore, true);
		assert_eq!(scTotalExpected, scTotalResult);
	}
	
	#[test]
	fn GlobalPercentage()
	{
		let mut instance = Game::default();
		instance.achievements.push(setupAchievement("A1", Platform::Steam, 0, 0, None));
		
		assert!(instance.isGlobalPercentageMissing(Platform::Steam));
		instance.achievements[0].platforms.iter_mut().for_each(|d| d.globalPercentage = Some(25.0));
		assert!(!instance.isGlobalPercentageMissing(Platform::Steam));
	}
}
