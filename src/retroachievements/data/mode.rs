use serde::{Deserialize, Serialize};
use serde_json::Number;

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

impl From<Number> for AchievementMode
{
	fn from(value: Number) -> Self
	{
		return match value.as_u64()
		{
			None => Self::default(),
			Some(num) => match num
			{
				0 => Self::Casual,
				1 => Self::Hardcore,
				_ => Self::default(),
			}
		};
	}
}
