use ::serde::{Deserialize, Serialize};
use crate::data::SteamAchievement;

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
	pub fn getAchievements(&self) -> Vec<SteamAchievement>
	{
		let mut achievements = vec![];
		if let Some(a) = &self.game.availableGameStats.achievements
		{
			for ga in a
			{
				let achievement = SteamAchievement::from(ga.to_owned());
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
