use std::collections::HashMap;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::battlenet::platform::data::starcraft2::profile::metadata::AchievementMetadata;
use crate::battlenet::platform::data::starcraft2::profile::profile::EarnedAchievement;
use super::criteria::Sc2Criteria;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Sc2Achievement
{
	pub criteria: Vec<Sc2Criteria>,
	pub description: String,
	pub displayOrder: u64,
	pub id: u64,
	pub name: String,
	pub points: u64,
	pub unlocked: bool,
	
    #[serde(with = "ts_seconds_option")]
	pub unlockedTimestamp: Option<DateTime<Utc>>,
}

impl From<AchievementMetadata> for Sc2Achievement
{
	fn from(value: AchievementMetadata) -> Self
	{
		return Self
		{
			description: value.description.clone(),
			displayOrder: value.uiOrderHint,
			
			id: value.id.parse::<u64>()
				.unwrap_or_default(),
			
			name: value.title.clone(),
			points: value.points,
			..Default::default()
		};
	}
}

impl PartialOrd for Sc2Achievement
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return self.displayOrder.partial_cmp(&other.displayOrder);
	}
}

impl Sc2Achievement
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Option<Self>
	{
		let mut achievement = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "achievements")
		{
			if let Value::Array(inner) = value
			{
				let mut criteriaList = vec![];
				
				for value in inner
				{
					if let Value::Object(criteriaValues) = value
					{
						if let Some(criteria) = Sc2Criteria::parseJsonMapLossy(criteriaValues)
						{
							criteriaList.push(criteria);
						}
					}
				}
				
				achievement.criteria = criteriaList;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "description")
		{
			if let Value::String(inner) = value
			{
				achievement.description = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "displayOrder")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.displayOrder = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.id = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				achievement.name = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "points")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					achievement.points = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "unlocked")
		{
			if let Value::Bool(inner) = value
			{
				achievement.unlocked = *inner;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "unlockedTimestamp")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_i64()
				{
					achievement.unlockedTimestamp = DateTime::from_timestamp(number, 0);
				}
			}
		}
		
		return match achievement.id > 0
		{
			false => None,
			true => Some(achievement),
		};
	}
	
	/**
	Process the achievements metadata returned by the Profile/Static endpoint.
	
	Returns a map pairing each `Sc2Achievement` with its associated category id.
	*/
	pub fn processMetadata(metadata: Vec<AchievementMetadata>) -> HashMap<String, Self>
	{
		let mut map = HashMap::new();
		
		for metaChievo in metadata
		{
			map.insert(
				metaChievo.categoryId.clone(),
				Self::from(metaChievo),
			);
		}
		
		return map;
	}
	
	pub fn update(&mut self, other: &Sc2Achievement)
	{
		self.description = other.description.clone();
		self.displayOrder = other.displayOrder;
		self.id = other.id;
		self.name = other.name.clone();
		self.points = other.points;
		self.unlocked = other.unlocked;
		self.unlockedTimestamp = other.unlockedTimestamp.clone();
	}
	
	pub fn updateEarned(&mut self, value: EarnedAchievement)
	{
		self.unlocked = value.isComplete;
		self.unlockedTimestamp = DateTime::from_timestamp(value.completionDate as i64, 0);
	}
	
	pub fn updateStatic(&mut self, other: AchievementMetadata)
	{
		self.description = other.description.clone();
		self.displayOrder = other.uiOrderHint;
		self.name = other.title.clone();
		self.points = other.points;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const JsonPayload: &str = r#"
[
	{
		"categoryId":"1",
		"chainAchievementIds":["2"],
		"chainRewardSize":1,
		"criteriaIds":["3"],
		"description":"Achievement id 4 description",
		"flags":2,
		"id":"4",
		
		"imageUrl":"someurl",
		
		"isChained":false,
		"points":0,
		"title":"Achievement id 4",
		"uiOrderHint":8
	},
	
	{
		"categoryId":"5",
		"chainAchievementIds":["6"],
		"chainRewardSize":0,
		"criteriaIds":["7"],
		"description":"Achievement id 8 description",
		"flags":2,
		"id":"8",
		"imageUrl":"someurl",
		"isChained":false,
		"points":0,
		"title":"Achievement id 8",
		"uiOrderHint":12
	},
	
	{
		"categoryId":"9",
		"chainAchievementIds":["10"],
		"chainRewardSize":0,
		"criteriaIds":[
			"11",
			"12",
			"13"
		],
		"description":"Achievement id 14 description",
		"flags":0,
		"id":"14",
		"imageUrl":"someurl",
		"isChained":false,
		"points":10,
		"title":"Achievement id 14",
		"uiOrderHint":27
	}
]
"#;
	
	const EarnedJson: &str = r#"
{
	"achievementId":"9",
	"completionDate":1765083932,
	"numCompletedAchievementsInSeries":10,
	"totalAchievementsInSeries":10,
	"isComplete":true,
	"inProgress":false,
	"criteria":[
		{"criterionId":"25","earned":{"quantity":1,"startTime":1}}
	]
}
"#;
	
	#[test]
	fn processMetadata()
	{
		let metadata = serde_json::from_str(JsonPayload);
		
		assert!(metadata.is_ok());
		let metadata = metadata.unwrap();
		
		let achievements = Sc2Achievement::processMetadata(metadata);
		assert!(!achievements.is_empty());
		assert_eq!(achievements.len(), 3);
		assert!(achievements.keys().all(|k| !k.is_empty()));
		
		let achievement = achievements.get("9");
		assert!(achievement.is_some());
		let achievement = achievement.unwrap();
		assert_eq!(achievement.id, 14);
		assert_eq!(&achievement.description, "Achievement id 14 description");
		assert_eq!(achievement.displayOrder, 27);
		assert_eq!(&achievement.name, "Achievement id 14");
		assert_eq!(achievement.points, 10);
		assert!(!achievement.unlocked);
	}
	
	#[test]
	fn update()
	{
		let dateTime = DateTime::parse_from_str(
			"20251207T050532+0000",
			"%Y%m%dT%H%M%S%z"
		)
			.unwrap()
			.naive_utc();
		
		let metadata = serde_json::from_str(EarnedJson);
		assert!(metadata.is_ok());
		
		let mut achievement = Sc2Achievement::default();
		assert!(!achievement.unlocked);
		assert_eq!(achievement.unlockedTimestamp, None);
		
		achievement.updateEarned(metadata.unwrap());
		assert!(achievement.unlocked);
		assert_eq!(achievement.unlockedTimestamp.unwrap().naive_utc(), dateTime);
	}
}
