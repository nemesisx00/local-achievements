use std::collections::VecDeque;
use freya::radio::RadioChannel;
use freya::winit::dpi::PhysicalSize;
use crate::settings::AppSettings;

use super::ActiveContent;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DataChannel
{
	ActiveContent,
	#[allow(unused)]
	Notifications,
	ProfileState,
	RateLimiter,
	Settings,
	WindowSize,
}

impl RadioChannel<AppSettings> for DataChannel {}
impl RadioChannel<Option<ActiveContent>> for DataChannel {}
impl RadioChannel<Option<bool>> for DataChannel {}
impl RadioChannel<PhysicalSize<u32>> for DataChannel {}
impl RadioChannel<VecDeque<String>> for DataChannel {}
