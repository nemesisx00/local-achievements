#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

/**
The expected response data returned by the GetPlayerAchievements (v0001) endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetPlayerAchievementsPayload
{
	pub playerstats: PlayerStats,
}

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerAchievement
{
	pub apiname: String,
	pub achieved: usize,
	pub unlocktime: usize,
	pub name: String,
	pub description: String,
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
