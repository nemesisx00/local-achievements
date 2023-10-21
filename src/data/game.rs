#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use ::serde::{Deserialize, Serialize};
use crate::platforms::Platform;
use super::achievement::{Achievement, Mode};

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
	
	/// The platform-specific IDs of this game.
	pub ids: HashMap<Platform, String>,
	
	/// The title of this game.
	pub name: String,
}

impl Game
{
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
			if achievement.platforms.contains(&Platform::RetroAchievements)
			{
				if let Some(info) = achievement.details.iter().find(|a| a.platform == Platform::RetroAchievements)
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
		}
		
		return points;
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	use crate::data::PlatformInfo;
	use crate::data::achievement::Mode;
	
	fn setupAchievement(name: &str, platform: Platform, hcPoints: usize, scPoints: usize, mode: Option<Mode>) -> Achievement
	{
		let mut achievement = Achievement::default();
		achievement.name = name.to_string();
		achievement.platforms.push(platform);
		
		let mut points = HashMap::new();
		points.insert(Mode::Hardcore, hcPoints);
		points.insert(Mode::Softcore, scPoints);
		
		achievement.details.push(PlatformInfo
		{
			globalPercentage: None,
			icon: None,
			mode: mode,
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
    fn Game_RetroPoints()
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
}
