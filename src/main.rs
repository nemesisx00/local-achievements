/*!
Local Achievements is an open source desktop application for collecting, storing,
and tracking your achievements across multiple platforms in one unified UI.
*/

mod components;
mod data;
mod io;
mod macros;
mod platforms;
mod constants;

use std::collections::VecDeque;
use freya::launch::launch_cfg;
use freya::prelude::{GlobalSignal, LaunchConfig, Signal, WindowConfig};
use crate::components::{ActiveContent, App};
use crate::data::{RetroAchievementsUser, SteamUser};
use crate::platforms::retroachievements::data::RetroAchievementsAuth;
use crate::platforms::steam::SteamAuth;
use crate::constants::{AppTitle, BackgroundColor, DefaultWindowSize, MinimumWindowSize};

pub static ActiveContent: GlobalSignal<ActiveContent> = Signal::global(|| Default::default());
pub static Language: GlobalSignal<String> = Signal::global(|| "en".to_string());
pub static NotificationList: GlobalSignal<VecDeque<String>> = Signal::global(|| Default::default());
pub static RetroAchievementsAuthData: GlobalSignal<RetroAchievementsAuth> = Signal::global(|| Default::default());
pub static RetroAchievementsUserData: GlobalSignal<RetroAchievementsUser> = Signal::global(|| Default::default());
pub static SelectedGameId: GlobalSignal<Option<usize>> = Signal::global(|| None);
pub static SteamAuthData: GlobalSignal<SteamAuth> = Signal::global(|| Default::default());
pub static SteamUserData: GlobalSignal<SteamUser> = Signal::global(|| Default::default());

fn main()
{
	launch_cfg(
		LaunchConfig::new()
			.with_window(
				WindowConfig::new(App)
					.with_background(BackgroundColor)
					.with_min_size(MinimumWindowSize.0, MinimumWindowSize.1)
					.with_size(DefaultWindowSize.0, DefaultWindowSize.1)
					.with_title(AppTitle)
					.with_transparency(false)
			)
	);
}
