use serde::{Deserialize, Serialize};
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
	pub games: Vec<Game>,
	
	/// The path to the user's avatar
	pub avatar: Option<String>,
	
	/// The user's 64-bit Steam ID
	pub id: String,
	
	/// The user's current publicly visible display name.
	pub name: String,
}

impl User
{
	pub const FileName: &str = "steam.json";
	
	pub fn update(&mut self, id: &String, name: &String, avatar: Option<&String>)
	{
		self.id = id.to_owned();
		self.name = name.to_owned();
		self.avatar = avatar.cloned();
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
}
