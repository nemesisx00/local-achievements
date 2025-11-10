use serde::{Deserialize, Serialize};

/**
The expected response data returned by the GetRecentlyPlayedGames endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload
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
