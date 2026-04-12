use data::enums::GamePlatforms;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};
use crate::data::region::Region;

/**
The data necessary to access the Steam Web API.
*/
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BattleNetSettings
{
	/// The user's preferred region for accessing the API
	pub defaultRegion: Region,
	/// The port to use when constructing the Redirect URI
	pub redirectPort: u64,
}

impl Default for BattleNetSettings
{
	fn default() -> Self
	{
		return Self
		{
			defaultRegion: Default::default(),
			redirectPort: 8080,
		};
	}
}

impl RadioChannel<BattleNetSettings> for GamePlatforms {}

impl BattleNetSettings
{
	/// The filename to be used when this struct is read from, or written to, the file system.
	pub const FileName: &str = "battlenet.json";
}
