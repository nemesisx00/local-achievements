use serde::{Deserialize, Serialize};
use crate::steam::platform::GameInfo;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Playtime
{
	pub linux: usize,
	pub mac: usize,
	pub offline: usize,
	pub total: usize,
	pub windows: usize,
}

impl Playtime
{
	pub fn update(&mut self, info: &GameInfo)
	{
		self.linux = info.playtime_linux_forever;
		self.mac = info.playtime_mac_forever;
		self.offline = info.playtime_disconnected;
		self.total = info.playtime_forever;
		self.windows = info.playtime_windows_forever;
	}
}
