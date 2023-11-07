#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

/**
The data returned by GetOwnedGames describing a single game.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameInfo
{
	pub appid: usize,
	pub has_community_visible_stats: Option<bool>,
	pub img_icon_url: String,
	pub name: String,
	pub playtime_disconnected: usize,
	pub playtime_forever: usize,
	pub playtime_linux_forever: usize,
	pub playtime_mac_forever: usize,
	pub playtime_windows_forever: usize,
	pub rtime_last_played: usize,
}

/**
The expected response data returned by the GetOwnedGames endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetOwnedGamesPayload
{
	pub response: OwnedGames,
}

/**
The count and list of games returned from the GetOwnedGames endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OwnedGames
{
	pub game_count: usize,
	pub games: Vec<GameInfo>,
}
