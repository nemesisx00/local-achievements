use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Default, Copy, Clone, Deserialize, EnumString, PartialEq, Serialize)]
pub enum Region
{
	#[strum(serialize = "China", serialize = "cn")]
	China,
	#[strum(serialize = "Europe", serialize = "eu")]
	Europe,
	#[strum(serialize = "Korea", serialize = "kr")]
	Korea,
	LatAm,
	Russia,
	#[strum(serialize = "Taiwan", serialize = "tn")]
	Taiwan,
	#[default]
	#[strum(serialize = "US", serialize = "us")]
	US,
}

impl Region
{
	pub fn from(regionId: u64, realmId: u64) -> Self
	{
		return match realmId
		{
			2 => match regionId
			{
				2 => Self::Russia,
				3 => Self::Taiwan,
				_ => Self::LatAm,
			},
			_ => match regionId
			{
				2 => Self::Europe,
				3 => Self::Korea,
				5 => Self::China,
				_ => Self::US,
			},
		};
	}
	
	pub fn realmId(&self) -> u64
	{
		return match self
		{
			Self::China | Self::Europe | Self::Korea | Self::US => 1,
			Self::LatAm | Self::Russia | Self::Taiwan => 2,
		};
	}
	
	pub fn regionId(&self) -> u64
	{
		return match self
		{
			Self::LatAm | Self::US => 1,
			Self::Europe | Self::Russia => 2,
			Self::Korea | Self::Taiwan => 3,
			Self::China => 5,
		};
	}
	
	pub fn shortString(&self) -> String
	{
		return match self
		{
			Self::China => "cn".into(),
			Self::Europe | Self::Russia => "eu".into(),
			Self::Korea => "kr".into(),
			Self::Taiwan => "tw".into(),
			Self::LatAm | Self::US => "us".into(),
		};
	}
}

#[cfg(test)]
mod tests
{
	use std::str::FromStr;
	use super::*;
	
	fn testData() -> Vec<(String, Result<Region, strum::ParseError>)>
	{
		return vec![
			("Europe".to_string(), Ok(Region::Europe)),
			("eu".to_string(), Ok(Region::Europe)),
			("LatAm".to_string(), Ok(Region::LatAm)),
			("China".to_string(), Ok(Region::China)),
			("cn".to_string(), Ok(Region::China)),
		];
	}
	
	#[test]
	fn deserialization()
	{
		for (value, expected) in testData()
		{
			assert_eq!(Region::from_str(value.as_str()), expected);
		}
		
		assert!(Region::from_str("la").is_err());
	}
}
