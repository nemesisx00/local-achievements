use serde::{Deserialize, Serialize};
use crate::data::retroachievements::makeRelative;
use crate::data::retroachievements::mode::AchievementMode;
use crate::data::retroachievements::rank::RankData;
use crate::data::retroachievements::game::Game;
use crate::platforms::retroachievements::data::{Payload_GetUserCompletionProgress, Payload_GetUserProfile};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User
{
	/// The relative path to the user's avatar on RetroAchievements.org.
	pub avatar: Option<String>,
	
	/// The list of games the player has played.
	pub games: Vec<Game>,
	
	/// The user's rank data for Hardcore mode.
	pub hardcore: RankData,
	
	/// The user's rank data for Softcore mode.
	pub softcore: RankData,
	
	/// The user's ULID.
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
			games: vec![],
			hardcore: RankData::default(),
			softcore: AchievementMode::Softcore.into(),
			ulid: None,
			username: String::default(),
		};
	}
}

impl User
{
	pub const Filename: &str = "retroAchievements.json";
	
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
		self.softcore.points = payload.TotalSoftcorePoints;
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
