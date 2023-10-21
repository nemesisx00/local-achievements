#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};
use super::achievement::Mode;
use super::game::Game;

/**
A single user, containing platform-specific profile information and its combined
list of games which have achievements.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct User
{
	/**
	The list of games associated with this user which also have achievements
	defined, across all platforms.
	*/
	pub games: Vec<Game>,
	
	/// This user's RetroAchievements profile information.
	pub retroAchievements: RetroAchievementsInfo,
	
	/// This user's Steam profile information.
	pub steam: SteamInfo,
}

/**
Profile information for a RetroAchievements user.
*/
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievementsInfo
{
	/// The user's username
	pub username: String,
	
	pub hardcore: RetroAchievementsRank,
	
	pub softcore: RetroAchievementsRank,
}

impl Default for RetroAchievementsInfo
{
	fn default() -> Self
	{
		return Self
		{
			username: String::new(),
			hardcore: RetroAchievementsRank::default(),
			softcore: RetroAchievementsRank::new(Mode::Softcore),
		};
	}
}

/**

*/
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievementsRank
{
	/// The mode corresponding to this rank and point amount.
	pub mode: Mode,
	
	/// The total number of points earned.
	pub points: usize,
	
	/// The current rank on RetroAchievements.org.
	pub rank: usize,
	
	/// The total users, used to create a relation for the rank.
	pub total: usize,
}

impl Default for RetroAchievementsRank
{
	fn default() -> Self
	{
		return Self
		{
			mode: Mode::Hardcore,
			points: 0,
			rank: 0,
			total: 0,
		}
	}
}

impl RetroAchievementsRank
{
	/**
	Create a new instance of RetroAchievementsRank with the given mode.
	*/
	pub fn new(mode: Mode) -> Self
	{
		return Self
		{
			mode,
			..Default::default()
		};
	}
}

/**
Profile information for a Steam user.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamInfo
{
	/// The path to the user's avatar
	pub avatar: Option<String>,
	
	/// The user's 64-bit Steam ID
	pub id: String,
	
	/// The user's current publicly visible display name.
	pub name: String,
}
