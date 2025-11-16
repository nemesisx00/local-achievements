use serde::{Deserialize, Serialize};

const DefaultNotificationDuration: u64 = 1000;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AppSettings
{
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
			notificationDuration: DefaultNotificationDuration,
		};
	}
}

impl AppSettings
{
	pub const FileName: &str = "config.json";
}
