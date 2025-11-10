use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Settings
{
	pub accountId: u64,
	/// The fully qualified path to the RPCS3 app data directory.
	pub appDataDirectory: String,
}

impl Settings
{
	pub const FileName: &str = "rpcs3.json";
}
