use std::cmp::Ordering;
use data::enums::GamePlatforms;

#[derive(Clone, Debug, Eq, PartialEq, Ord)]
pub enum DataOperation
{
	CacheImage(bool),
	Platform(GamePlatforms, String),
	PlatformGameId(GamePlatforms, String, u64),
	PlatformGameIdBool(GamePlatforms, String, u64, bool),
	PlatformGameIdString(GamePlatforms, String, String),
	PlatformOptionalInt(GamePlatforms, String, Option<u64>),
	PlatformSaveToFile(GamePlatforms),
	PlatformThreeInt(GamePlatforms, String, u64, u64, u64),
}

impl Default for DataOperation
{
	fn default() -> Self
	{
		return Self::CacheImage(false);
	}
}

impl PartialOrd for DataOperation
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self
		{
			DataOperation::CacheImage(_) => match other
			{
				DataOperation::CacheImage(_) => Some(Ordering::Equal),
				_ => Some(Ordering::Greater),
			},
			
			_ => match other
			{
				DataOperation::CacheImage(_) => Some(Ordering::Less),
				_ => Some(Ordering::Equal),
			},
		};
	}
}
