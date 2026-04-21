use data::enums::GamePlatforms;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SteamSettings
{
	#[serde(default)]
	pub enableSteamFamilyLibrary: bool,
}

impl RadioChannel<SteamSettings> for GamePlatforms {}

impl SteamSettings
{
	/// The filename to be used when this struct is read from, or written to, the file system.
	pub const FileName: &str = "steam.json";
}
