use std::str::FromStr;
use anyhow::{Error, anyhow};
use data::enums::GamePlatforms;
use net::{DataOperation, DataRequest};
use strum_macros::{AsRefStr, EnumString};
use crate::data::progress::RetroAchievementsProgressState;

#[derive(AsRefStr, Clone, Debug, EnumString, Eq, PartialEq, PartialOrd, Ord)]
pub enum RetroAchievementsOperation
{
	GetGameInfo(u64),
	GetUserProfile,
	GetUserProgress(RetroAchievementsProgressState),
	SaveToFile,
}

impl From<RetroAchievementsOperation> for DataOperation
{
	fn from(value: RetroAchievementsOperation) -> Self
	{
		return match value
		{
			RetroAchievementsOperation::GetGameInfo(gameId)
				=> DataOperation::PlatformGameId(
					GamePlatforms::RetroAchievements,
					RetroAchievementsOperation::GetGameInfo(0).as_ref().to_string(),
					gameId
				),
			
			RetroAchievementsOperation::GetUserProfile
				=> DataOperation::Platform(
					GamePlatforms::RetroAchievements,
					RetroAchievementsOperation::GetUserProfile.as_ref().to_string()
				),
			
			RetroAchievementsOperation::GetUserProgress(progress)
				=> DataOperation::PlatformThreeInt(
					GamePlatforms::RetroAchievements,
					RetroAchievementsOperation::GetUserProgress(Default::default()).as_ref().to_string(),
					progress.offset,
					progress.received,
					progress.total
				),
			
			RetroAchievementsOperation::SaveToFile => DataOperation::PlatformSaveToFile(GamePlatforms::RetroAchievements),
		};
	}
}

impl From<RetroAchievementsOperation> for DataRequest
{
	fn from(value: RetroAchievementsOperation) -> Self
	{
		return Self
		{
			operation: value.into(),
			..Default::default()
		};
	}
}

impl TryFrom<DataOperation> for RetroAchievementsOperation
{
	type Error = Error;
	
	fn try_from(value: DataOperation) -> Result<Self, Self::Error>
	{
		return match value
		{
			DataOperation::Platform(platform, operationName)
				=> match platform
				{
					GamePlatforms::RetroAchievements => Ok(RetroAchievementsOperation::from_str(&operationName)?),
					_ => Err(anyhow!("Invalid Retro Achievement operation")),
				}
			
			DataOperation::PlatformGameId(platform, operationName, gameId)
				=> match platform
				{
					GamePlatforms::RetroAchievements => match RetroAchievementsOperation::from_str(&operationName)?
					{
						Self::GetGameInfo(_) => Ok(Self::GetGameInfo(gameId)),
						_ => Err(anyhow!("Invalid Retro Achievement operation")),
					},
					_ => Err(anyhow!("Invalid Retro Achievement operation")),
				}
			
			DataOperation::PlatformSaveToFile(platform)
				=> match platform
				{
					GamePlatforms::RetroAchievements => Ok(RetroAchievementsOperation::SaveToFile),
					_ => Err(anyhow!("Invalid Retro Achievement operation")),
				}
			
			DataOperation::PlatformThreeInt(
				platform,
				operationName,
				offset,
				received,
				total
			)
				=> match platform
				{
					GamePlatforms::RetroAchievements => match RetroAchievementsOperation::from_str(&operationName)?
					{
						Self::GetUserProgress(_) => Ok(Self::GetUserProgress(RetroAchievementsProgressState
						{
							offset,
							received,
							total,
						})),
						_ => Err(anyhow!("Invalid Retro Achievement operation")),
					},
					_ => Err(anyhow!("Invalid Retro Achievement operation")),
				}
			
			_ => Err(anyhow!("Invalid Retro Achievement operation")),
		};
	}
}
