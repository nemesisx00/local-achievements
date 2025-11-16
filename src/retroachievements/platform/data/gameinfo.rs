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
	pub DisplayOrder: u64,
	pub ID: u64,
	pub MemAddr: String,
	pub NumAwarded: u64,
	pub NumAwardedHardcore: u64,
	pub Points: u64,
	pub Title: String,
	pub TrueRatio: u64,
	pub Type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payload
{
	pub Achievements: HashMap<u64, AchievementMetadata>,
	pub ConsoleID: u64,
	pub ConsoleName: String,
	pub Developer: String,
	pub Flags: u64,
	pub ForumTopicID: u64,
	pub Genre: String,
	pub HighestAwardDate: Option<String>,
	pub HighestAwardKind: Option<String>,
	pub ID: u64,
	pub ImageBoxArt: String,
	pub ImageIcon: String,
	pub ImageIngame: String,
	pub ImageTitle: String,
	pub IsFinal: bool,
	pub NumAchievements: u64,
	pub NumAwardedToUser: u64,
	pub NumAwardedToUserHardcore: u64,
	pub NumDistinctPlayers: u64,
	pub NumDistinctPlayersCasual: u64,
	pub NumDistinctPlayersHardcore: u64,
	pub ParentGameID: Option<u64>,
	pub Publisher: String,
	pub Released: String,
	pub ReleasedAtGranularity: String,
	pub RichPresencePatch: String,
	pub Title: String,
	pub UserCompletion: String,
	pub UserCompletionHardcore: String,
}
