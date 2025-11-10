use serde::{Deserialize, Serialize};

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerAchievement
{
	pub apiname: String,
	pub achieved: usize,
	pub unlocktime: usize,
	pub name: String,
	pub description: Option<String>,
}

/**
The player stats for a given game.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerStats
{
	pub steamID: String,
	pub gameName: String,
	pub achievements: Vec<PlayerAchievement>,
}

/**
The expected response data returned by the GetPlayerAchievements (v0001) endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload
{
	pub playerstats: PlayerStats,
}
