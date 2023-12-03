#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};
use crate::platforms::steam::SteamGame;

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamInfo
{
	pub id: usize,
	pub iconHash: String,
	pub lastPlayed: usize,
	pub playtime: SteamPlaytime,
}

impl SteamInfo
{
	pub fn new(info: SteamGame) -> Self
	{
		let mut instance = Self::default();
		instance.update(info);
		return instance;
	}
	
	pub fn update(&mut self, info: SteamGame)
	{
		self.id = info.appid;
		self.iconHash = info.img_icon_url.to_owned();
		self.lastPlayed = info.rtime_last_played;
		self.playtime.update(info);
	}
}

/**

*/
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamPlaytime
{
	pub linux: usize,
	pub mac: usize,
	pub offline: usize,
	pub total: usize,
	pub windows: usize,
}

impl SteamPlaytime
{
	pub fn update(&mut self, info: SteamGame)
	{
		self.linux = info.playtime_linux_forever;
		self.mac = info.playtime_mac_forever;
		self.offline = info.playtime_disconnected;
		self.total = info.playtime_forever;
		self.windows = info.playtime_windows_forever;
	}
}
