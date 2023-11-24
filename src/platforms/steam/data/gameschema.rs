#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameStat
{
	pub name: String,
	pub defaultvalue: isize,
	pub displayName: String,
}
