use serde::{Deserialize, Serialize};

/**
The data necessary to access the Steam Web API.
*/
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BattleNetSettings
{
	/// The port to use when constructing the Redirect URI
	pub redirectPort: u64,
}

impl Default for BattleNetSettings
{
	fn default() -> Self
	{
		return Self
		{
			redirectPort: 8080,
		};
	}
}

impl BattleNetSettings
{
	/// The filename to be used when this struct is read from, or written to, the file system.
	pub const FileName: &str = "battlenet.json";
}
