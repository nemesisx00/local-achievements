use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProfileStarcraft2
{
	pub id: u64,
	pub realm: u64,
	pub name: String,
	pub totalAchievementPoints: u64,
	pub totalSwarmLevel: u64,
}
