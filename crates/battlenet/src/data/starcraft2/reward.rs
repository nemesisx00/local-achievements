use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::api::RewardMetadata;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Sc2Reward
{
	pub displayOrder: u64,
	pub id: u64,
	pub name: String,
}

impl From<RewardMetadata> for Sc2Reward
{
	fn from(value: RewardMetadata) -> Self
	{
		return Self
		{
			displayOrder: value.uiOrderHint,
			
			id: value.id.parse::<u64>()
				.unwrap_or_default(),
			
			name: value.name.clone(),
		};
	}
}

impl Sc2Reward
{
	/**
	Process the rewards metadata returned by the Profile/Static endpoint.
	
	Returns a map pairing each `Sc2Reward` with its associated achievement id.
	*/
	pub fn processMetadata(metadata: Vec<RewardMetadata>) -> HashMap<String, Self>
	{
		let mut map = HashMap::new();
		
		for metaReward in metadata
		{
			if let Some(achievementId) = metaReward.achievementId.clone()
			{
				let reward = Self::from(metaReward);
				map.insert(achievementId, reward);
			}
		}
		
		return map;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const JsonPayload: &str = r#"
[
	{
		"flags":0,
		"id":"1",
		"achievementId":"2",
		"name":"Reward id 1",
		"imageUrl":"someurl",
		"unlockableType":"PCls",
		"isSkin":true,
		"uiOrderHint":5
	},
	
	{
		"flags":6,
		"id":"3",
		"achievementId":"4",
		"name":"Reward id 3",
		"imageUrl":"someurl",
		"unlockableType":"ZCAL",
		"isSkin":false,
		"uiOrderHint":8
	}
]
"#;
	
	#[test]
	fn processMetadata()
	{
		let metadata = serde_json::from_str(JsonPayload);
		
		assert!(metadata.is_ok());
		let metadata = metadata.unwrap();
		
		let rewards = Sc2Reward::processMetadata(metadata);
		assert!(!rewards.is_empty());
		assert_eq!(rewards.len(), 2);
		assert!(rewards.keys().all(|k| !k.is_empty()));
		
		let reward = rewards.get("4");
		assert!(reward.is_some());
		let achievement = reward.unwrap();
		assert_eq!(achievement.id, 3);
		assert_eq!(achievement.displayOrder, 8);
		assert_eq!(&achievement.name, "Reward id 3");
	}
}
