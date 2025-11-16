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
	pub total_count: u64,
	pub games: Vec<RecentlyPlayed>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RecentlyPlayed
{
	pub appid: u64,
	pub name: String,
	pub playtime_2weeks: u64,
	pub playtime_forever: u64,
	pub img_icon_url: String,
	pub playtime_windows_forever: u64,
	pub playtime_mac_forever: u64,
	pub playtime_linux_forever: u64,
}
