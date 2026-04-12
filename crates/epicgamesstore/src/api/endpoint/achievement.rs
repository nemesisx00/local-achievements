use serde::{Deserialize, Serialize};
use super::Variables;
use super::unified::Rarity;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload_Achievement
{
	pub data: AchievementData,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AchievementSet
{
	pub achievementSetId: String,
	pub isBase: bool,
	pub totalAchievements: u64,
	pub totalXP: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AchievementData
{
	pub Achievement: AchievementContainer,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AchievementContainer
{
	pub productAchievementsRecordBySandbox: Achievements,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Achievements
{
	pub productId: String,
	pub sandboxId: String,
	pub totalAchievements: u64,
	pub totalProductXP: u64,
	pub achievementSets: Vec<AchievementSet>,
	pub platinumRarity: Rarity,
	pub achievements: Vec<AchievementProgressContainer>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AchievementProgressContainer
{
	pub achievement: Achievement,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Achievement
{
	pub sandboxId: String,
	pub deploymentId: String,
	pub name: String,
	pub hidden: bool,
	pub isBase: bool,
	pub achievementSetId: String,
	pub unlockedDisplayName: String,
	pub lockedDisplayName: String,
	pub unlockedDescription: String,
	pub lockedDescription: String,
	pub unlockedIconId: String,
	pub lockedIconId: String,
	pub XP: u64,
	pub flavorText: String,
	pub unlockedIconLink: String,
	pub lockedIconLink: String,
	pub tier: Tier,
	pub rarity: Rarity,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tier
{
	pub name: String,
	pub hexColor: String,
	pub min: u64,
	pub max: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct AchievementVariables
{
	pub sandboxId: String,
	pub locale: String,
}

impl Default for AchievementVariables
{
	fn default() -> Self
	{
		return Self
		{
			//TODO: Set up localization for the app
			locale: "en=US".into(),
			sandboxId: Default::default(),
		};
	}
}

impl Variables for AchievementVariables {}
