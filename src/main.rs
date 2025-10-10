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

use freya::launch::launch_cfg;
use freya::prelude::{GlobalSignal, LaunchConfig, Signal, WindowConfig};
use crate::components::App;
use crate::data::{RetroAchievementsUser, SteamUser};
use crate::platforms::retroachievements::data::RetroAchievementsAuth;
use crate::platforms::steam::SteamAuth;
use crate::constants::BackgroundColor;

pub const AppTitle: &str = "Local Achievements";
pub const AppVersion: &str = "0.2.0";
pub const DefaultWindowSize: (f64, f64) = (1280.0, 720.0);
pub const MinimumWindowSize: (f64, f64) = (720.0, 480.0);

pub static RetroAchievementsAuthData: GlobalSignal<RetroAchievementsAuth> = Signal::global(|| RetroAchievementsAuth::default());
pub static RetroAchievementsUserData: GlobalSignal<RetroAchievementsUser> = Signal::global(|| RetroAchievementsUser::default());
pub static SteamAuthData: GlobalSignal<SteamAuth> = Signal::global(|| SteamAuth::default());
pub static SteamUserData: GlobalSignal<SteamUser> = Signal::global(|| SteamUser::default());

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
