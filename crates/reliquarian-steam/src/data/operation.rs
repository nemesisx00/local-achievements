use std::str::FromStr;
use anyhow::{Error, anyhow};
use data::enums::GamePlatforms;
use net::{DataOperation, DataRequest};
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, Clone, Copy, Debug, Eq, EnumString, PartialEq, PartialOrd, Ord)]
pub enum SteamOperation
{
	GetGameList,
	GetGlobalPercentages(u64),
	GetGameImage(u64, bool),
	GetPlayerAchievements(u64),
	GetPlayerSummary,
	GetSchemaForGame(u64),
	SaveToFile,
	SetGameLoaded(u64, bool),
}

impl From<SteamOperation> for DataOperation
{
	fn from(value: SteamOperation) -> Self
	{
		return match value
		{
			SteamOperation::GetGameList
				| SteamOperation::GetPlayerSummary
			=> DataOperation::Platform(
				GamePlatforms::Steam,
				value.as_ref().to_string()
			),
			
			SteamOperation::GetGlobalPercentages(gameId)
				| SteamOperation::GetPlayerAchievements(gameId)
				| SteamOperation::GetSchemaForGame(gameId)
			=> DataOperation::PlatformGameId(
				GamePlatforms::Steam,
				value.as_ref().to_string(),
				gameId
			),
			
			SteamOperation::SaveToFile => DataOperation::PlatformSaveToFile(GamePlatforms::Steam),
			
			SteamOperation::SetGameLoaded(gameId, switch)
				| SteamOperation::GetGameImage(gameId, switch)
			=> DataOperation::PlatformGameIdBool(
				GamePlatforms::Steam,
				value.as_ref().to_string(),
				gameId,
				switch
			),
		};
	}
}

impl From<SteamOperation> for DataRequest
{
	fn from(value: SteamOperation) -> Self
	{
		return Self
		{
			operation: value.into(),
			..Default::default()
		}
	}
}

impl TryFrom<DataOperation> for SteamOperation
{
	type Error = Error;
	
	fn try_from(value: DataOperation) -> Result<Self, Self::Error>
	{
		return match value
		{
			DataOperation::Platform(platform, operationName)
				=> match platform
				{
					GamePlatforms::Steam => match SteamOperation::from_str(&operationName)?
					{
						Self::GetGameList => Ok(Self::GetGameList),
						Self::GetPlayerSummary => Ok(Self::GetPlayerSummary),
						_ => Err(anyhow!("Invalid Steam operation")),
					},
					_ => Err(anyhow!("Invalid Steam operation")),
				}
			
			DataOperation::PlatformGameId(platform, operationName, gameId)
				=> match platform
				{
					GamePlatforms::Steam => match SteamOperation::from_str(&operationName)?
					{
						Self::GetGlobalPercentages(_) => Ok(Self::GetGlobalPercentages(gameId)),
						Self::GetPlayerAchievements(_) => Ok(Self::GetPlayerAchievements(gameId)),
						Self::GetSchemaForGame(_) => Ok(Self::GetSchemaForGame(gameId)),
						_ => Err(anyhow!("Steam operation parameter mismatch")),
					}
					
					_ => Err(anyhow!("Invalid Steam operation")),
				}
			
			DataOperation::PlatformGameIdBool(platform, operationName, gameId, switch)
				=> match platform
				{
					GamePlatforms::Steam => match SteamOperation::from_str(&operationName)?
					{
						Self::GetGameImage(_, _) => Ok(Self::GetGameImage(gameId, switch)),
						Self::SetGameLoaded(_, _) => Ok(Self::SetGameLoaded(gameId, switch)),
						_ => Err(anyhow!("Steam operation parameter mismatch")),
					}
					
					_ => Err(anyhow!("Invalid Steam operation")),
				}
			
			DataOperation::PlatformSaveToFile(platform)
				=> match platform
				{
					GamePlatforms::Steam => Ok(Self::SaveToFile),
					_ => Err(anyhow!("Invalid Steam operation"))
				}
			
			_ => Err(anyhow!("Invalid Steam operation"))
		}
	}
}
