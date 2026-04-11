use std::str::FromStr;
use anyhow::{Error, anyhow};
use data::enums::GamePlatforms;
use net::{DataOperation, DataRequest};
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, Clone, Debug, EnumString, Eq, PartialEq, PartialOrd, Ord)]
pub enum GogOperation
{
	GetAchievements(u64),
	GetFilteredProducts(Option<u64>),
	GetUserInfo,
	RefreshSession,
	SaveToFile,
}

impl From<GogOperation> for DataOperation
{
	fn from(value: GogOperation) -> Self
	{
		return match value
		{
			GogOperation::GetAchievements(gameId) => DataOperation::PlatformGameId(
				GamePlatforms::Gog,
				value.as_ref().to_string(),
				gameId
			),
			
			GogOperation::GetFilteredProducts(page) => DataOperation::PlatformOptionalInt(
				GamePlatforms::Gog,
				value.as_ref().to_string(),
				page
			),
			
			GogOperation::GetUserInfo
				| GogOperation::RefreshSession
			=> DataOperation::Platform(
				GamePlatforms::Gog,
				value.as_ref().to_string()
			),
			
			GogOperation::SaveToFile => DataOperation::PlatformSaveToFile(GamePlatforms::Gog),
		};
	}
}

impl From<GogOperation> for DataRequest
{
	fn from(value: GogOperation) -> Self
	{
		return Self
		{
			operation: value.into(),
			..Default::default()
		};
	}
}

impl TryFrom<DataOperation> for GogOperation
{
	type Error = Error;
	
	fn try_from(value: DataOperation) -> Result<Self, Self::Error>
	{
		return match value
		{
			DataOperation::Platform(platform, operationName) => match platform
			{
				GamePlatforms::Gog => match GogOperation::from_str(&operationName)?
				{
					Self::GetUserInfo => Ok(Self::GetUserInfo),
					Self::RefreshSession => Ok(Self::RefreshSession),
					
					_ => Err(anyhow!("Invalid GOG operation")),
				},
				
				_ => Err(anyhow!("Invalid GOG operation")),
			}
			
			DataOperation::PlatformGameId(
				platform,
				operationName,
				gameId
			) => match platform
			{
				GamePlatforms::Gog => match GogOperation::from_str(&operationName)?
				{
					Self::GetAchievements(_) => Ok(Self::GetAchievements(gameId)),
					_ => Err(anyhow!("Invalid GOG operation")),
				},
				
				_ => Err(anyhow!("Invalid GOG operation")),
			}
			
			DataOperation::PlatformOptionalInt(
				platform,
				operationName,
				page
			) => match platform
			{
				GamePlatforms::Gog => match GogOperation::from_str(&operationName)?
				{
					Self::GetFilteredProducts(_) => Ok(Self::GetFilteredProducts(page)),
					_ => Err(anyhow!("Invalid GOG operation")),
				},
				
				_ => Err(anyhow!("Invalid GOG operation")),
			}
			
			DataOperation::PlatformSaveToFile(platform) => match platform
			{
				GamePlatforms::Gog => Ok(Self::SaveToFile),
				_ => Err(anyhow!("Invalid GOG operation")),
			}
			
			_ => Err(anyhow!("Invalid GOG operation")),
		};
	}
}
