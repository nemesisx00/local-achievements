use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Settings
{
	#[serde(default)]
	pub accountId: u64,
	
	/// The fully qualified path to the RPCS3 app data directory.
	#[serde(default)]
	pub appDataDirectory: String,
}

impl Settings
{
	pub const FileName: &str = "rpcs3.json";
}
