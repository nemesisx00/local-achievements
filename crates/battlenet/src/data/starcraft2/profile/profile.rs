use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::api::{PayloadPlayer, PayloadProfile, PayloadStatic};
use crate::data::region::Region;
use crate::data::starcraft2::achievement::Sc2Achievement;
use crate::data::starcraft2::profile::campaign::CampaignsSummary;
use crate::data::starcraft2::profile::career::CareerSummary;
use crate::data::starcraft2::profile::levels::FactionLevel;
use crate::data::starcraft2::profile::snapshot::Snapshot;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProfileStarcraft2
{
	pub achievements: Vec<Sc2Achievement>,
	pub career: CareerSummary,
	pub campaigns: CampaignsSummary,
	pub id: u64,
	pub leagueSnapshot: Snapshot,
	pub levelProtoss: FactionLevel,
	pub levelTerran: FactionLevel,
	pub levelTotal: u64,
	pub levelZerg: FactionLevel,
	pub region: Region,
	pub name: String,
	pub totalAchievementPoints: u64,
}

impl From<PayloadPlayer> for ProfileStarcraft2
{
	fn from(value: PayloadPlayer) -> Self
	{
		return Self
		{
			id: value.profileId
				.parse::<u64>().unwrap_or_default(),
			
			name: value.name.clone(),
			region: Region::fromRegionRealm(value.regionId, value.realmId),
			..Default::default()
		};
	}
}

impl ProfileStarcraft2
{
	pub fn getAchievement(&self, id: u64) -> Option<Sc2Achievement>
	{
		return self.achievements.iter()
			.find(|a| a.id == id)
			.cloned();
	}
	
	pub fn getFilteredAchievements(&self, text: impl Into<String>) -> Vec<Sc2Achievement>
	{
		let searchText = text.into().to_lowercase();
		return self.achievements.iter()
			.filter(|a| a.name.to_lowercase().contains(&searchText)
				|| a.description.to_lowercase().contains(&searchText))
			.cloned()
			.collect();
	}
	
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Option<Self>
	{
		let mut profile = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "achievements")
		{
			if let Value::Array(inner) = value
			{
				let mut achievements = vec![];
				
				for value in inner
				{
					if let Value::Object(achievementValues) = value
					{
						if let Some(achievement) = Sc2Achievement::parseJsonMapLossy(achievementValues)
						{
							achievements.push(achievement);
						}
						
					}
				}
				
				profile.achievements = achievements;
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "career")
		{
			if let Value::Object(inner) = value
			{
				profile.career = CareerSummary::parseJsonMapLossy(inner)
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "campaigns")
		{
			if let Value::Object(inner) = value
			{
				profile.campaigns = CampaignsSummary::parseJsonMapLossy(inner)
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					profile.id = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "leagueSnapshot")
		{
			if let Value::Object(inner) = value
			{
				profile.leagueSnapshot = Snapshot::parseJsonMapLossy(inner)
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "levelProtoss")
		{
			if let Value::Object(inner) = value
			{
				profile.levelProtoss = FactionLevel::parseJsonMapLossy(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "levelTerran")
		{
			if let Value::Object(inner) = value
			{
				profile.levelTerran = FactionLevel::parseJsonMapLossy(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "levelTotal")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					profile.levelTotal = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "levelZerg")
		{
			if let Value::Object(inner) = value
			{
				profile.levelZerg = FactionLevel::parseJsonMapLossy(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "name")
		{
			if let Value::String(inner) = value
			{
				profile.name = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "region")
		{
			if let Value::String(inner) = value
			{
				profile.region = inner.into();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalAchievementPoints")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					profile.totalAchievementPoints = number;
				}
			}
		}
		
		return match profile.id > 0
		{
			false => None,
			true => Some(profile),
		};
	}
	
	pub fn updateAccount(&mut self, payload: PayloadPlayer)
	{
		self.id = payload.profileId
			.parse::<u64>().unwrap_or_default();
		
		self.name = payload.name.clone();
		self.region = Region::fromRegionRealm(payload.regionId, payload.realmId);
	}
	
	pub fn updateProfile(&mut self, payload: PayloadProfile)
	{
		self.totalAchievementPoints = payload.summary.totalAchievementPoints;
		
		self.campaigns = payload.campaign.difficultyCompleted.into();
		self.career = payload.career.into();
		self.leagueSnapshot = payload.snapshot.into();
		self.levelProtoss = payload.swarmLevels.protoss.into();
		self.levelTerran = payload.swarmLevels.terran.into();
		self.levelTotal = payload.swarmLevels.level;
		self.levelZerg = payload.swarmLevels.zerg.into();
		
		for earned in payload.earnedAchievements
		{
			if let Some(achievement) = self.achievements.iter_mut()
				.find(|a| a.id.to_string() == earned.achievementId)
			{
				achievement.updateEarned(earned);
			}
		}
	}
	
	pub fn updateStatic(&mut self, payload: PayloadStatic)
	{
		for metadata in payload.achievements
		{
			match self.achievements.iter_mut()
				.find(|a| a.id.to_string() == metadata.id)
			{
				None => self.achievements.push(metadata.into()),
				Some(achievement) => achievement.updateStatic(metadata),
			}
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
		"id": 7,
		"region": "kr",
		"name": "The StarCraft II name",
		"totalAchievementPoints": 20
	}
"#;
	
	#[test]
	fn parseJsonMapLossy()
	{
		let root = serde_json::from_str::<Value>(PartialJson);
		assert!(root.is_ok());
		
		let value = root.unwrap();
		assert!(value.is_object());
		
		let map = value.as_object();
		assert!(map.is_some());
		
		let result = ProfileStarcraft2::parseJsonMapLossy(&map.unwrap());
		assert!(result.is_some());
		
		let profile = result.unwrap();
		assert_eq!(profile.id, 7);
		assert_eq!(profile.region, Region::Korea);
		assert_eq!(&profile.name, "The StarCraft II name");
		assert_eq!(profile.totalAchievementPoints, 20);
		assert_eq!(profile.levelTotal, 0);
	}
}
