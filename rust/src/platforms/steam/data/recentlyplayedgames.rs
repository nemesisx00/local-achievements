#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetRecentlyPlayedGamesPayload
{
	pub response: RecentlyPlayedGames,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RecentlyPlayedGames
{
	pub total_count: usize,
	pub games: Vec<RecentlyPlayed>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RecentlyPlayed
{
	pub appid: usize,
	pub name: String,
	pub playtime_2weeks: usize,
	pub playtime_forever: usize,
	pub img_icon_url: String,
	pub playtime_windows_forever: usize,
	pub playtime_mac_forever: usize,
	pub playtime_linux_forever: usize,
}
