use data::enums::GamePlatforms;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RetroAchievementsSettings
{
	#[serde(default)]
	pub showGameAwardBadges: bool,
}

impl Default for RetroAchievementsSettings
{
	fn default() -> Self
	{
		return Self
		{
			showGameAwardBadges: true,
		};
	}
}

impl RadioChannel<RetroAchievementsSettings> for GamePlatforms {}

impl RetroAchievementsSettings
{
	/// The filename to be used when this struct is read from, or written to, the file system.
	pub const FileName: &str = "retroachievements.json";
}
