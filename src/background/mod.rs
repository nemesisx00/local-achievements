#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod dispatch;

pub use dispatch::Dispatcher;

use crate::platforms::steam::SteamAuth;

/**
The type of command the background task will receive from the frontend.
*/
#[derive(Clone, Debug, PartialEq)]
pub enum ApiCommand
{
	Metadata(Internal),
	Print(String),
	Steam(SteamEndpoint),
}

/**
The type of response payload the frontend will receive from the background task.
*/
#[derive(Clone, Debug, PartialEq)]
pub enum CommandResponse
{
	Refresh,
	SteamAuth(SteamAuth),
}

/**
API commands for interacting with internal application metadata.
*/
#[derive(Clone, Debug, PartialEq)]
pub enum Internal
{
	GetSteamAuth,
	SaveUserData,
	SteamAuth(SteamAuth),
	UpdateSteamId(String),
	UpdateSteamApiKey(String),
}

/**
API commands for interacting with the Steam API.
*/
#[derive(Clone, Debug, PartialEq)]
pub enum SteamEndpoint
{
	GlobalPercentages(usize),
	OwnedGames,
	PlayerAchievements(usize, String),
	PlayerSummaries,
	RecentlyPlayedGames,
	SchemaForGame(usize, String),
}
