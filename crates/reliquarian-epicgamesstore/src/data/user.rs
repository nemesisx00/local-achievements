use anyhow::Result;
use data::enums::GamePlatforms;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::{Payload_PlayerProfile, Payload_PlayerProfilePrivate};
use crate::data::achievement::EgsAchievement;
use super::game::EgsGame;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EgsUser
{
	pub accountId: String,
	
	#[serde(default)]
	pub games: Vec<EgsGame>,
	
	#[serde(default)]
	pub name: String,
}

impl RadioChannel<EgsUser> for GamePlatforms {}

impl EgsUser
{
	pub const FileName: &str = "egs.json";
	
	pub fn filterGames(&self, search: impl Into<String>) -> Vec<EgsGame>
	{
		let text = search.into().to_lowercase();
		let mut games = self.games.iter()
			.filter(|g| g.name.to_lowercase().contains(&text))
			.cloned()
			.collect::<Vec<_>>();
		games.sort();
		
		return games;
	}
	
	pub fn getAchievement(&self, sandboxId: &String, achievementId: &String) -> Option<EgsAchievement>
	{
		return match self.getGame(sandboxId)
		{
			None => None,
			Some(g) => g.achievements.iter()
				.find(|a| &a.id == achievementId)
				.cloned(),
		};
	}
	
	pub fn getGame(&self, sandboxId: &String) -> Option<EgsGame>
	{
		return self.games.iter()
			.find(|g| &g.sandboxId == sandboxId)
			.cloned();
	}
	
	/**
	Parse a JSON string which does not strictly conform to the expected `User`
	data structure.
	
	This function will retain as much data as possible but will omit objects if
	they are missing required properties.
	
	Any missing properties which are not strictly required will instead be
	filled with their default values.
	
	Only returns `Err` if `json` is not valid JSON.
	*/
	pub fn parseJsonLossy(json: String) -> Result<Self>
	{
		let root = serde_json::from_str::<Value>(json.as_str())?;
		
		let mut user = Self::default();
		
		match root
		{
			Value::Object(map) => {
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "accountId")
				{
					if let Value::String(value) = value
					{
						user.accountId = value.clone();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "games")
				{
					if let Value::Array(games) = value
					{
						let mut parsedGames = vec![];
						for gameValue in games
						{
							if let Value::Object(gameMap) = gameValue
							{
								if let Some(game) = EgsGame::parseJsonMap(gameMap)
								{
									parsedGames.push(game);
								}
							}
						}
						user.games = parsedGames;
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "name")
				{
					if let Value::String(value) = value
					{
						user.name = value.clone();
					}
				}
			},
			
			_ => {},
		}
		
		return Ok(user);
	}
	
	pub fn updateProfile(&mut self, payload: Payload_PlayerProfile)
	{
		self.accountId = payload.data.PlayerProfile.playerProfile.epicAccountId.clone();
		self.name = payload.data.PlayerProfile.playerProfile.displayName.clone();
	}
	
	pub fn updateProfilePrivate(&mut self, payload: Payload_PlayerProfilePrivate)
	{
		for metadata in payload.data.PlayerProfile.playerProfile.achievementsSummaries.data
		{
			match self.games.iter_mut()
				.find(|g| g.sandboxId == metadata.sandboxId)
			{
				None => self.games.push(metadata.into()),
				Some(game) => game.updateSummary(metadata),
			}
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
	"accountId": "The account id",
	"name": "Test User",
	"games": [
		{
			"sandboxId": "The first game id",
			"achievements": [
				{ "id": "The first achievement", "unlocked": { "name": "Successful parse!" } },
				{ "unlocked": { "name": "This one should fail to parse" } }
			]
		},
		
		{
			"name": "Test game that shouldn't parse",
			"achievements": [
				{ "id": "The last achievement", "unlocked": { "name": "Successful parse!" } }
			]
		}
	]
}"#;
	
	#[test]
	fn parseJsonLossy()
	{
		let result = EgsUser::parseJsonLossy(PartialJson.into());
		assert!(result.is_ok());
		
		let user = result.unwrap();
		assert_eq!(&user.accountId, "The account id");
		assert_eq!(&user.name, "Test User");
		assert_eq!(user.games.len(), 1);
		
		let game = user.games.first().unwrap();
		assert_eq!(&game.sandboxId, "The first game id");
		assert_eq!(game.achievements.len(), 1);
		
		let achievement = game.achievements.first().unwrap();
		assert_eq!(&achievement.id, "The first achievement");
		assert_eq!(&achievement.unlocked.name, "Successful parse!");
	}
}
