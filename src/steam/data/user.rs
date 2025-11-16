use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::steam::platform::Payload_GetOwnedGames;
use super::game::Game;

/**
Profile information for a Steam user.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct User
{
	/**
	The list of games associated with this user which also have achievements
	defined, across all platforms.
	*/
	#[serde(default)]
	pub games: Vec<Game>,
	
	/// The path to the user's avatar
	#[serde(default)]
	pub avatar: Option<String>,
	
	/// The user's 64-bit Steam ID
	pub id: String,
	
	/// The user's current publicly visible display name.
	#[serde(default)]
	pub name: String,
}

impl User
{
	pub const FileName: &str = "steam.json";
	
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
					.find(|(k, _)| k.as_str() == "games")
				{
					if let Value::Array(inner) = value
					{
						let mut parsedGames = vec![];
						for gameValue in inner
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
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "avatar")
				{
					if let Value::String(inner) = value
					{
						if !inner.is_empty()
						{
							user.avatar = Some(inner.to_owned());
						}
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "id")
				{
					if let Value::String(inner) = value
					{
						user.id = inner.to_owned();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "name")
				{
					if let Value::String(value) = value
					{
						user.name = value.to_owned();
					}
				}
			},
			
			_ => {},
		}
		
		return Ok(user);
	}
	
	pub fn processOwnedGames(&mut self, payload: Payload_GetOwnedGames)
	{
		for game in payload.response.games
		{
			match self.games.iter_mut()
				.find(|g| g.id == game.appid)
			{
				None => self.games.push(game.clone().into()),
				Some(g) => g.update(&game),
			}
		}
	}
	
	pub fn update(&mut self, id: &String, name: &String, avatar: Option<&String>)
	{
		self.id = id.to_owned();
		self.name = name.to_owned();
		self.avatar = avatar.cloned();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
	"games": [
		{
			"id": 73,
			"name": "First game",
			"achievements": [
				{ "id": "chievo1", "name": "Successful parse!" },
				{ "name": "This one should fail to parse" }
			]
		},
		
		{
			"name": "Test game that shouldn't parse",
			"achievements": [
				{ "id": 4, "name": "Successful parse!" }
			]
		}
	],
	"avatar": "The avatar",
	"id": "The id",
	"name": "The name"
}"#;
	
	#[test]
	fn parseJsonLossy()
	{
		let result = User::parseJsonLossy(PartialJson.into());
		assert!(result.is_ok());
		
		let user = result.unwrap();
		assert_eq!(user.avatar, Some("The avatar".to_string()));
		assert_eq!(user.id, "The id".to_string());
		assert_eq!(user.name, "The name".to_string());
		assert_eq!(user.games.len(), 1);
		
		let game = user.games.first().unwrap();
		assert_eq!(game.id, 73);
		assert_eq!(game.name, "First game".to_string());
		assert_eq!(game.achievements.len(), 1);
		
		let trophy = game.achievements.first().unwrap();
		assert_eq!(trophy.id, "chievo1".to_string());
		assert_eq!(trophy.name, "Successful parse!".to_string());
	}
}
