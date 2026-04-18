use serde::{Deserialize, Serialize};
use crate::enums::ActiveContent;
use super::platforms::EnabledPlatforms;
use super::Language;

pub const DefaultNotificationDuration: u64 = 1000;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AppSettings
{
	#[serde(default)]
	pub defaultActivePlatform: ActiveContent,
	
	#[serde(default)]
	pub enabledPlatforms: EnabledPlatforms,
	
	#[serde(default)]
	pub language: Language,
	
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
			enabledPlatforms: Default::default(),
			language: Default::default(),
			notificationDuration: DefaultNotificationDuration,
		};
	}
}

impl AppSettings
{
	pub const FileName: &str = "config.json";
}
