use std::str::FromStr;
use anyhow::{Error, anyhow};
use data::enums::GamePlatforms;
use net::{DataOperation, DataRequest};
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, Clone, Debug, EnumString, Eq, PartialEq, PartialOrd, Ord)]
pub enum BattleNetOperation
{
	GetSc2PlayerAccount,
	GetSc2PlayerProfile,
	GetSc2StaticProfile,
	GetUserInfo,
	SaveToFile,
}

impl From<BattleNetOperation> for DataOperation
{
	fn from(value: BattleNetOperation) -> Self
	{
		return match value
		{
			BattleNetOperation::SaveToFile => DataOperation::PlatformSaveToFile(GamePlatforms::BattleNet),
			_ => DataOperation::Platform(GamePlatforms::BattleNet, value.as_ref().to_string())
		}
	}
}

impl From<BattleNetOperation> for DataRequest
{
	fn from(value: BattleNetOperation) -> Self
	{
		return Self
		{
			operation: value.into(),
			..Default::default()
		};
	}
}

impl TryFrom<DataOperation> for BattleNetOperation
{
	type Error = Error;
	
	fn try_from(value: DataOperation) -> Result<Self, Self::Error>
	{
		return match value
		{
			DataOperation::Platform(platform, operationName)
				=> match platform
				{
					GamePlatforms::BattleNet => match BattleNetOperation::from_str(&operationName)?
					{
						Self::GetSc2PlayerAccount => Ok(Self::GetSc2PlayerAccount),
						Self::GetSc2PlayerProfile => Ok(Self::GetSc2PlayerProfile),
						Self::GetSc2StaticProfile => Ok(Self::GetSc2StaticProfile),
						Self::GetUserInfo => Ok(Self::GetUserInfo),
						_ => Err(anyhow!("Invalid Battle.Net operation"))
					}
					
					_ => Err(anyhow!("Invalid Battle.Net operation"))
				}
			
			DataOperation::PlatformSaveToFile(platform) => match platform
			{
				GamePlatforms::BattleNet => Ok(Self::SaveToFile),
				_ => Err(anyhow!("Invalid Battle.Net operation"))
			}
			
			_ => Err(anyhow!("Invalid Battle.Net operation"))
		};
	}
}
