use serde::{Deserialize, Serialize};

/**
The mode representing conditions under which an achievement was unlocked.

*Only used by: RetroAchievements*
*/
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum AchievementMode
{
	#[default]
	Casual,
	Hardcore,
}
