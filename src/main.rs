/*!
Local Achievements is an open source desktop application for collecting, storing,
and tracking your achievements across multiple platforms in one unified UI.
*/

mod components;
mod data;
mod io;
mod macros;
mod constants;
mod retroachievements;
mod rpcs3;
mod steam;
mod util;

use std::collections::VecDeque;
use std::path::Path;
use freya::launch::launch_cfg;
use freya::prelude::{GlobalSignal, LaunchConfig, Signal};
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

use crate::components::{ActiveContent, App};
use crate::data::AppSettings;
use crate::constants::{AppTitle, BackgroundColor, DefaultWindowSize, MinimumWindowSize};
use crate::io::{FileName_LogPrefix, Path_Logs, getConfigDir};
use crate::retroachievements::{RetroAchievementsAuth, RetroAchievementsUser};
use crate::rpcs3::{Rpcs3Settings, Rpcs3User};
use crate::steam::{SteamAuth, SteamUser};

pub static ActiveContent: GlobalSignal<ActiveContent> = Signal::global(|| Default::default());
pub static GameSelected: GlobalSignal<bool> = Signal::global(|| false);
pub static Language: GlobalSignal<String> = Signal::global(|| "en".to_string());
pub static NotificationList: GlobalSignal<VecDeque<String>> = Signal::global(|| Default::default());
pub static RetroAchievementsAuthData: GlobalSignal<RetroAchievementsAuth> = Signal::global(|| Default::default());
pub static RetroAchievementsUserData: GlobalSignal<RetroAchievementsUser> = Signal::global(|| Default::default());
pub static Rpcs3SettingsData: GlobalSignal<Rpcs3Settings> = Signal::global(|| Default::default());
pub static Rpcs3UserData: GlobalSignal<Rpcs3User> = Signal::global(|| Default::default());
pub static Settings: GlobalSignal<AppSettings> = Signal::global(|| Default::default());
pub static SteamAuthData: GlobalSignal<SteamAuth> = Signal::global(|| Default::default());
pub static SteamUserData: GlobalSignal<SteamUser> = Signal::global(|| Default::default());

fn main()
{
	let _guard = configureLogger();
	
	launch_cfg(App, LaunchConfig::<()>::new()
		.with_background(BackgroundColor)
		.with_min_size(MinimumWindowSize.0, MinimumWindowSize.1)
		.with_size(DefaultWindowSize.0, DefaultWindowSize.1)
		.with_title(AppTitle)
		.with_transparency(false)
	);
	
	/*
	launch_cfg(
		LaunchConfig::new()
			.with_window(WindowConfig::new(App)
				.with_background(BackgroundColor)
				.with_min_size(MinimumWindowSize.0, MinimumWindowSize.1)
				.with_size(DefaultWindowSize.0, DefaultWindowSize.1)
				.with_title(AppTitle)
				.with_transparency(false))
	);
	*/
}

fn configureLogger() -> WorkerGuard
{
	let dir = getConfigDir(true).unwrap();
	
	let logPath = Path::new(&dir)
		.join(Path_Logs);
	
	let fileAppender = tracing_appender::rolling::daily(logPath, FileName_LogPrefix);
	let (nonBlocking, workerGuard) = tracing_appender::non_blocking(fileAppender);
	
	let format = tracing_subscriber::fmt::format()
		.with_ansi(false)
		.compact();
	
	tracing_subscriber::fmt()
		.event_format(format)
		.with_max_level(Level::INFO)
		.with_writer(nonBlocking)
		.init();
	
	return workerGuard;
}
