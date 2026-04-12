use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::api::CampaignDifficultyComplete;
use crate::data::starcraft2::enums::DifficultyLevel;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CampaignsSummary
{
	pub heartOfTheSwarm: Option<DifficultyLevel>,
	pub legacyOfTheVoid: Option<DifficultyLevel>,
	pub wingsOfLiberty: Option<DifficultyLevel>,
}

impl From<CampaignDifficultyComplete> for CampaignsSummary
{
	fn from(value: CampaignDifficultyComplete) -> Self
	{
		return Self
		{
			heartOfTheSwarm: match DifficultyLevel::from_str(&value.heartOfTheSwarm)
			{
				Err(_) => None,
				Ok(dl) => Some(dl),
			},
			
			legacyOfTheVoid: match DifficultyLevel::from_str(&value.legacyOfTheVoid)
			{
				Err(_) => None,
				Ok(dl) => Some(dl),
			},
			
			wingsOfLiberty: match DifficultyLevel::from_str(&value.wingsOfLiberty)
			{
				Err(_) => None,
				Ok(dl) => Some(dl),
			},
		};
	}
}

impl CampaignsSummary
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Self
	{
		let mut summary = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "wingsOfLiberty")
		{
			if let Value::String(inner) = value
			{
				if let Ok(difficulty) = DifficultyLevel::from_str(inner)
				{
					summary.wingsOfLiberty = Some(difficulty);
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "heartOfTheSwarm")
		{
			if let Value::String(inner) = value
			{
				if let Ok(difficulty) = DifficultyLevel::from_str(inner)
				{
					summary.heartOfTheSwarm = Some(difficulty);
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "legacyOfTheVoid")
		{
			if let Value::String(inner) = value
			{
				if let Ok(difficulty) = DifficultyLevel::from_str(inner)
				{
					summary.legacyOfTheVoid = Some(difficulty);
				}
			}
		}
		
		return summary;
	}
}
