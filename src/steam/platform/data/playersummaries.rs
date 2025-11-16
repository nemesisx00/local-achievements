use serde::{Deserialize, Serialize};

/**
The expected response data returned by the GetPlayerSummaries endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload
{
	pub response: PlayerSummaries,
}

/**
A list of users' profile info, as returned by the GetPlayerSummaries endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerSummaries
{
	pub players: Vec<PlayerSummary>,
}

/**
A single user's profile info, as returned by the GetPlayerSummaries endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerSummary
{
	pub steamid: String,
	pub communityvisibilitystate: u64,
	pub profilestate: u64,
	pub personaname: String,
	pub profileurl: String,
	pub avatar: String,
	pub avatarmedium: String,
	pub avatarfull: String,
	pub avatarhash: String,
	pub lastlogoff: u64,
	pub personastate: u64,
	pub realname: String,
	pub primaryclanid: String,
	pub timecreated: u64,
	pub personastateflags: u64,
	pub loccountrycode: String,
}
