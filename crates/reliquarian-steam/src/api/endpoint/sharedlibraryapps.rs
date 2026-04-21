use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AppInfo
{
	pub appid: u64,
	pub owner_steamids: Vec<String>,
	pub name: String,
	pub capsule_filename: String,
	pub img_icon_hash: Option<String>,
	pub exclude_reason: u64,
	pub rt_time_acquired: u64,
	pub rt_last_played: u64,
	pub rt_playtime: u64,
	pub app_type: u64,
}

/**
The expected response data returned by the SharedLibraryApps endpoint.
*/
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Payload_GetSharedLibraryApps
{
	pub response: SharedApps,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SharedApps
{
	pub apps: Vec<AppInfo>,
	pub owner_steamid: String,
}
