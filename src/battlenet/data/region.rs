use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};

#[derive(AsRefStr, Debug, Default, Copy, Clone, Deserialize, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub enum Region
{
	China,
	Europe,
	Korea,
	LatAm,
	Russia,
	Taiwan,
	#[default]
	US,
}

impl From<String> for Region
{
	fn from(value: String) -> Self
	{
		return match value.to_lowercase().as_str()
		{
			"china" => Self::China,
			"cn" => Self::China,
			"europe" => Self::Europe,
			"eu" => Self::Europe,
			"korea" => Self::Korea,
			"kr" => Self::Korea,
			"latam" => Self::LatAm,
			"russia" => Self::Russia,
			"taiwan" => Self::Taiwan,
			"tn" => Self::Taiwan,
			"us" => Self::US,
			_ => Self::default(),
		};
	}
}

impl From<&String> for Region
{
	fn from(value: &String) -> Self
	{
		return value.clone().into();
	}
}

impl From<&str> for Region
{
	fn from(value: &str) -> Self
	{
		return value.to_string().into();
	}
}

impl Region
{
	pub fn fromRegionRealm(regionId: u64, realmId: u64) -> Self
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
	use super::*;
	
	fn testData() -> Vec<(String, Region)>
	{
		return vec![
			("Europe".to_string(), Region::Europe),
			("eu".to_string(), Region::Europe),
			("LatAm".to_string(), Region::LatAm),
			("China".to_string(), Region::China),
			("cn".to_string(), Region::China),
		];
	}
	
	#[test]
	fn deserialization()
	{
		for (value, expected) in testData()
		{
			assert_eq!(Region::from(value), expected);
		}
		
		assert_eq!(Region::from("la"), Region::US);
	}
}
