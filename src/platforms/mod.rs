#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

pub mod retroachievements;
pub mod steam;
mod util;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Platform
{
	RetroAchievements,
	Steam,
}

impl Platform
{
	pub fn nameOf(platform: Platform) -> String
	{
		return match platform
		{
			Self::RetroAchievements => "Retro Achievements",
			Self::Steam => "Steam",
		}.to_string();
	}
}
