#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};
use crate::data::{Achievement, PlatformInfo};
use crate::platforms::Platform;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameAchievement
{
	pub name: String,
	pub defaultvalue: usize,
	pub displayName: String,
	pub hidden: usize,
	pub description: Option<String>,
	pub icon: String,
	pub icongray: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameAvailableStats
{
	pub achievements: Option<Vec<GameAchievement>>,
	pub stats: Option<Vec<GameStat>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameSchema
{
	pub availableGameStats: GameAvailableStats,
	pub gameName: String,
	pub gameVersion: String,
}

/**
The expected response data returned by the GetSchemaForGame endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetSchemaForGamePayload
{
	pub game: GameSchema,
}

impl GetSchemaForGamePayload
{
	pub fn getAchievements(&self) -> Vec<Achievement>
	{
		let mut achievements = vec![];
		if let Some(a) = &self.game.availableGameStats.achievements
		{
			for ga in a
			{
				let description = match ga.description.to_owned()
				{
					Some(d) => d,
					None => String::default(),
				};
				
				let mut pi = PlatformInfo::new(ga.name.to_owned(), ga.displayName.to_owned(), description, Platform::Steam);
				pi.icon = Some(ga.icon.to_owned());
				pi.iconLocked = Some(ga.icongray.to_owned());
				
				let mut achievement = Achievement::default();
				achievement.platforms.push(pi);
				
				achievements.push(achievement);
			}
		}
		
		return achievements;
	}
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameStat
{
	pub name: String,
	pub defaultvalue: isize,
	pub displayName: String,
}
