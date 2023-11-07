#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

/**
The expected response data returned by the GetPlayerSummaries endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetPlayerSummariesPayload
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
	pub communityvisibilitystate: usize,
	pub profilestate: usize,
	pub personaname: String,
	pub profileurl: String,
	pub avatar: String,
	pub avatarmedium: String,
	pub avatarfull: String,
	pub avatarhash: String,
	pub lastlogoff: usize,
	pub personastate: usize,
	pub realname: String,
	pub primaryclanid: String,
	pub timecreated: usize,
	pub personastateflags: usize,
	pub loccountrycode: String,
}
