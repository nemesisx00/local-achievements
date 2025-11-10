use serde::{Deserialize, Serialize};
use super::mode::AchievementMode;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RankData
{
	/// The mode corresponding to this rank and point amount.
	pub mode: AchievementMode,
	
	/// The total number of points earned.
	pub points: usize,
	
	/// The current rank on RetroAchievements.org.
	pub rank: usize,
	
	/// The total users, used to create a relation for the rank.
	pub total: usize,
}

impl From<AchievementMode> for RankData
{
	fn from(value: AchievementMode) -> Self
	{
		return Self
		{
			mode: value,
			..Default::default()
		};
	}
}
