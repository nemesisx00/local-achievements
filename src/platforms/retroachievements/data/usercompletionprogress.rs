use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameMetadata
{
	pub ConsoleID: usize,
	pub ConsoleName: String,
	pub GameID: usize,
	pub HighestAwardDate: Option<String>,
	pub HighestAwardKind: Option<String>,
	pub ImageIcon: String,
	pub MaxPossible: usize,
	pub MostRecentAwardedDate: Option<String>,
	pub NumAwarded: usize,
	pub NumAwardedHardcore: usize,
	pub Title: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payload
{
	pub Count: usize,
	pub Results: Vec<GameMetadata>,
	pub Total: usize,
}
