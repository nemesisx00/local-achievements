use std::str::FromStr;
use anyhow::{Error, anyhow};
use data::enums::GamePlatforms;
use net::{DataOperation, DataRequest};
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, Clone, Debug, EnumString, Eq, PartialEq, PartialOrd, Ord)]
pub enum EgsOperation
{
	GetAchievementsList(String),
	GetAchievementProgress(String),
	GetPlayerProfile,
	GetPlayerProfilePrivate,
	SaveToFile,
}

impl From<EgsOperation> for DataOperation
{
	fn from(value: EgsOperation) -> Self
	{
		return match value.clone()
		{
			EgsOperation::GetAchievementProgress(gameId)
				| EgsOperation::GetAchievementsList(gameId)
			=> DataOperation::PlatformGameIdString(
				GamePlatforms::EpicGamesStore,
				value.as_ref().to_string(),
				gameId
			),
			
			EgsOperation::GetPlayerProfile
				| EgsOperation::GetPlayerProfilePrivate
			=> DataOperation::Platform(
				GamePlatforms::EpicGamesStore,
				value.as_ref().to_string()
			),
			
			EgsOperation::SaveToFile => DataOperation::PlatformSaveToFile(GamePlatforms::EpicGamesStore),
		};
	}
}

impl From<EgsOperation> for DataRequest
{
	fn from(value: EgsOperation) -> Self
	{
		return Self
		{
			operation: value.into(),
			..Default::default()
		};
	}
}

impl TryFrom<DataOperation> for EgsOperation
{
	type Error = Error;
	
	fn try_from(value: DataOperation) -> Result<Self, Self::Error>
	{
		return match value
		{
			DataOperation::Platform(platform, operationName)
				=> match platform
				{
					GamePlatforms::EpicGamesStore => match EgsOperation::from_str(&operationName)?
					{
						Self::GetPlayerProfile => Ok(Self::GetPlayerProfile),
						Self::GetPlayerProfilePrivate => Ok(Self::GetPlayerProfilePrivate),
						_ => Err(anyhow!("Invalid Epic Games Store operation")),
					},
					
					_ => Err(anyhow!("Invalid Epic Games Store operation")),
				}
			
			DataOperation::PlatformGameIdString(platform, operationName, gameId)
				=> match platform
				{
					GamePlatforms::EpicGamesStore => match EgsOperation::from_str(&operationName)?
					{
						Self::GetAchievementProgress(_) => Ok(Self::GetAchievementProgress(gameId)),
						Self::GetAchievementsList(_) => Ok(Self::GetAchievementsList(gameId)),
						_ => Err(anyhow!("Invalid Epic Games Store operation")),
					},
					
					_ => Err(anyhow!("Invalid Epic Games Store operation")),
				}
			
			DataOperation::PlatformSaveToFile(platform)
				=> match platform
				{
					GamePlatforms::EpicGamesStore => Ok(Self::SaveToFile),
					_ => Err(anyhow!("Invalid Epic Games Store operation")),
				}
			
			_ => Err(anyhow!("Invalid Epic Games Store operation")),
		};
	}
}
