use serde::{Deserialize, Serialize};
use crate::data::ActiveContent;

pub const DefaultNotificationDuration: u64 = 1000;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AppSettings
{
	#[serde(default)]
	pub defaultActivePlatform: ActiveContent,
	
	#[serde(default)]
	pub language: String,
	
	/// The duration in milliseconds for which a notification should remain on screen.
	#[serde(default)]
	pub notificationDuration: u64,
}

impl Default for AppSettings
{
	fn default() -> Self
	{
		return Self
		{
			defaultActivePlatform: Default::default(),
			language: Default::default(),
			notificationDuration: DefaultNotificationDuration,
		};
	}
}

impl AppSettings
{
	pub const FileName: &str = "config.json";
}
