use std::collections::VecDeque;
use freya::radio::RadioChannel;
use freya::winit::dpi::PhysicalSize;
use crate::battlenet::BattleNetGames;
use crate::components::ProfileState;
use crate::data::ActiveContent;
use crate::data::data::AppData;
use crate::net::limiter::RateLimiter;
use crate::net::limiter::request::RequestEvent;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DataChannel
{
	ActiveContent,
	#[allow(unused)]
	Notifications,
	ProfileState,
	RateLimiter,
	WindowSize,
}

impl RadioChannel<Option<ActiveContent>> for DataChannel {}
impl RadioChannel<Option<bool>> for DataChannel {}
impl RadioChannel<PhysicalSize<u32>> for DataChannel {}
impl RadioChannel<ProfileState> for DataChannel {}
impl RadioChannel<RateLimiter> for DataChannel {}
impl RadioChannel<RequestEvent> for DataChannel {}
impl RadioChannel<VecDeque<String>> for DataChannel {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppDataChannel
{
	BattleNet,
	EpicGamesStore,
	Gog,
	RetroAchievements,
	Rpcs3,
	Settings,
	Steam,
}

impl RadioChannel<AppData> for AppDataChannel {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameIdChannel
{
	BattleNet,
	EpicGamesStore,
	Gog,
	RetroAchievements,
	Rpcs3,
	Steam,
}

impl RadioChannel<Option<BattleNetGames>> for GameIdChannel {}
impl RadioChannel<Option<String>> for GameIdChannel {}
impl RadioChannel<Option<u64>> for GameIdChannel {}
