use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::retroachievements::platform::{Payload_GetUserCompletionProgress,
	Payload_GetUserProfile};
use super::makeRelative;
use super::mode::AchievementMode;
use super::rank::RankData;
use super::game::Game;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User
{
	/// The relative path to the user's avatar on RetroAchievements.org.
	#[serde(default)]
	pub avatar: Option<String>,
	
	/// The user's rank data for Casual mode.
	#[serde(default)]
	pub casual: RankData,
	
	/// The list of games the player has played.
	#[serde(default)]
	pub games: Vec<Game>,
	
	/// The user's rank data for Hardcore mode.
	#[serde(default)]
	pub hardcore: RankData,
	
	/// The user's ULID.
	#[serde(default)]
	pub ulid: Option<String>,
	
	/// The user's username.
	pub username: String,
}

impl Default for User
{
	fn default() -> Self
	{
		return Self
		{
			avatar: None,
			casual: AchievementMode::Casual.into(),
			games: vec![],
			hardcore: RankData::default(),
			ulid: None,
			username: String::default(),
		};
	}
}

impl User
{
	pub const FileName: &str = "retroAchievements.json";
	
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
					.find(|(k, _)| k.as_str() == "avatar")
				{
					if let Value::String(value) = value
					{
						user.avatar = Some(value.to_owned());
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "casual")
				{
					if let Value::Object(rankMap) = value
					{
						user.casual = RankData::parseJsonMap(rankMap);
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "hardcore")
				{
					if let Value::Object(rankMap) = value
					{
						user.hardcore = RankData::parseJsonMap(rankMap);
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "ulid")
				{
					if let Value::String(value) = value
					{
						user.ulid = Some(value.to_owned());
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "username")
				{
					if let Value::String(value) = value
					{
						user.username = value.to_owned();
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
								if let Some(game) = Game::parseJsonMap(gameMap)
								{
									parsedGames.push(game);
								}
							}
						}
						user.games = parsedGames;
					}
				}
			},
			
			_ => {},
		}
		
		return Ok(user);
	}
	
	pub fn processUserCompletionProgress(&mut self, payload: &Payload_GetUserCompletionProgress)
	{
		for metadata in payload.Results.iter()
		{
			match self.games.iter_mut()
				.find(|g| g.id == metadata.GameID)
			{
				None => self.games.push(metadata.to_owned().into()),
				Some(game) => game.update(&metadata),
			}
		}
	}
	
	pub fn processUserProfile(&mut self, payload: &Payload_GetUserProfile)
	{
		self.casual.points = payload.TotalSoftcorePoints;
		self.hardcore.points = payload.TotalPoints;
		
		self.avatar = match payload.UserPic.is_empty()
		{
			false => Some(makeRelative(&payload.UserPic)),
			true => None,
		};
		
		self.ulid = match payload.ULID.is_empty()
		{
			false => Some(payload.ULID.to_owned()),
			true => None,
		};
		
		self.username = payload.User.to_owned();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
	"username": "Test User",
	"games": [
		{
			"id": 7,
			"achievements": [
				{ "id": 4, "name": "Successful parse!" },
				{ "name": "This one should fail to parse" }
			]
		},
		
		{
			"name": "Test game that shouldn't parse",
			"achievements": [
				{ "id": 4, "name": "Successful parse!" }
			]
		}
	]
}"#;
	
	#[test]
	fn parseJsonLossy()
	{
		let result = User::parseJsonLossy(PartialJson.into());
		assert!(result.is_ok());
		
		let user = result.unwrap();
		assert_eq!(user.games.len(), 1);
		
		let game = user.games.first().unwrap();
		assert_eq!(game.id, 7);
		assert_eq!(game.achievements.len(), 1);
		
		let achievement = game.achievements.first().unwrap();
		assert_eq!(achievement.id, 4);
		assert_eq!(achievement.name, "Successful parse!".to_string());
	}
}
