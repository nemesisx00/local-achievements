#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};
use crate::platforms::Platform;
use crate::platforms::steam::SteamGame;
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
}

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievementsInfo
{
	pub id: String,
}

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamInfo
{
	pub id: usize,
	pub iconHash: String,
	pub lastPlayed: usize,
	pub playtime: SteamPlaytime,
}

impl SteamInfo
{
	pub fn new(info: SteamGame) -> Self
	{
		let mut instance = Self::default();
		instance.update(info);
		return instance;
	}
	
	pub fn update(&mut self, info: SteamGame)
	{
		self.id = info.appid;
		self.iconHash = info.img_icon_url.to_owned();
		self.lastPlayed = info.rtime_last_played;
		self.playtime.update(info);
	}
}

/**

*/
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamPlaytime
{
	pub linux: usize,
	pub mac: usize,
	pub offline: usize,
	pub total: usize,
	pub windows: usize,
}

impl SteamPlaytime
{
	pub fn update(&mut self, info: SteamGame)
	{
		self.linux = info.playtime_linux_forever;
		self.mac = info.playtime_mac_forever;
		self.offline = info.playtime_disconnected;
		self.total = info.playtime_forever;
		self.windows = info.playtime_windows_forever;
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
