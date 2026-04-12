use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::Variables;
use super::unified::Avatars;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload_AchievementProgress
{
	pub data: PlayerProfileData,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AchievementSetProgress
{
	pub achievementSetId: String,
	pub isBase: bool,
	pub totalUnlocked: u64,
	pub totalXP: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfileData
{
	pub PlayerProfile: PlayerProfileContainer,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfileContainer
{
	pub playerProfile: PlayerProfile,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfile
{
	pub epicAccountId: String,
	pub displayName: String,
	pub relationship: String,
	pub avatar: Avatars,
	pub productAchievements: ProductAchievements,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProductAchievements
{
	pub __typename: String,
	pub data: ProductAchievementsData,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProductAchievementsData
{
	pub epicAccountId: String,
	pub sandboxId: String,
	pub totalXP: u64,
	pub totalUnlocked: u64,
	pub achievementSets: Vec<AchievementSetProgress>,
	pub playerAwards: Vec<Value>,
	pub playerAchievements: Vec<PlayerAchievementContainer>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerAchievementContainer
{
	pub playerAchievement: PlayerAchievement,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerAchievement
{
	pub achievementName: String,
	pub epicAccountId: String,
	pub progress: u64,
	pub sandboxId: String,
	pub unlocked: bool,
	pub unlockDate: String,
	pub XP: u64,
	pub achievementSetId: String,
	pub isBase: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AchievementProgressVariables
{
	pub epicAccountId: String,
	pub productId: String,
}

impl Variables for AchievementProgressVariables {}
