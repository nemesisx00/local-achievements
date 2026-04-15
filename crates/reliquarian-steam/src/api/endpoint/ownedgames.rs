use serde::Deserialize;

/**
The data returned by GetOwnedGames describing a single game.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct GameInfo
{
	pub appid: u64,
	pub has_community_visible_stats: Option<bool>,
	pub img_icon_url: String,
	pub name: String,
	pub playtime_disconnected: u64,
	pub playtime_forever: u64,
	pub playtime_linux_forever: u64,
	pub playtime_mac_forever: u64,
	pub playtime_windows_forever: u64,
	pub rtime_last_played: u64,
}

/**
The count and list of games returned from the GetOwnedGames endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct OwnedGames
{
	pub game_count: u64,
	pub games: Vec<GameInfo>,
}

/**
The expected response data returned by the GetOwnedGames endpoint.
*/
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Payload_GetOwnedGames
{
	pub response: OwnedGames,
}
