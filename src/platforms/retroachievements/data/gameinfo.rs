use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AchievementMetadata
{
	pub Author: String,
	pub AuthorULID: String,
	pub BadgeName: String,
	pub DateCreated: String,
	pub DateEarned: Option<String>,
	pub DateEarnedHardcore: Option<String>,
	pub DateModified: String,
	pub Description: String,
	pub DisplayOrder: usize,
	pub ID: usize,
	pub MemAddr: String,
	pub NumAwarded: usize,
	pub NumAwardedHardcore: usize,
	pub Points: usize,
	pub Title: String,
	pub TrueRatio: usize,
	pub Type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payload
{
	pub Achievements: HashMap<usize, AchievementMetadata>,
	pub ConsoleID: usize,
	pub ConsoleName: String,
	pub Developer: String,
	pub Flags: usize,
	pub ForumTopicID: usize,
	pub Genre: String,
	pub HighestAwardDate: Option<String>,
	pub HighestAwardKind: Option<String>,
	pub ID: usize,
	pub ImageBoxArt: String,
	pub ImageIcon: String,
	pub ImageIngame: String,
	pub ImageTitle: String,
	pub IsFinal: bool,
	pub NumAchievements: usize,
	pub NumAwardedToUser: usize,
	pub NumAwardedToUserHardcore: usize,
	pub NumDistinctPlayers: usize,
	pub NumDistinctPlayersCasual: usize,
	pub NumDistinctPlayersHardcore: usize,
	pub ParentGameID: Option<usize>,
	pub Publisher: String,
	pub Released: String,
	pub ReleasedAtGranularity: String,
	pub RichPresencePatch: String,
	pub Title: String,
	pub UserCompletion: String,
	pub UserCompletionHardcore: String,
}
