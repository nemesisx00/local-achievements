use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameMetadata
{
	pub ConsoleID: u64,
	pub ConsoleName: String,
	pub GameID: u64,
	pub HighestAwardDate: Option<String>,
	pub HighestAwardKind: Option<String>,
	pub ImageIcon: String,
	pub MaxPossible: u64,
	pub MostRecentAwardedDate: Option<String>,
	pub NumAwarded: u64,
	pub NumAwardedHardcore: u64,
	pub Title: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payload
{
	pub Count: u64,
	pub Results: Vec<GameMetadata>,
	pub Total: u64,
}
