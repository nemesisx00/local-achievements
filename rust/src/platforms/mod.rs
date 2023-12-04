use ::serde::{Deserialize, Serialize};

pub mod retroachievements;
pub mod steam;
mod util;

pub const Icon_Locked: &str = "locked";

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
