use ::serde::{Deserialize, Serialize};

/**
The data returned by GetOwnedGames describing a single game.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameInfo
{
	pub appid: i64,
	pub has_community_visible_stats: Option<bool>,
	pub img_icon_url: String,
	pub name: String,
	pub playtime_disconnected: i64,
	pub playtime_forever: i64,
	pub playtime_linux_forever: i64,
	pub playtime_mac_forever: i64,
	pub playtime_windows_forever: i64,
	pub rtime_last_played: i64,
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
	pub game_count: i64,
	pub games: Vec<GameInfo>,
}
