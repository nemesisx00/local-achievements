/*!
Local Achievements is an open source desktop application for collecting, storing,
and tracking your achievements across multiple platforms in one unified UI.
*/

//Disable the additional command prompt window when running the application on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;

use std::path::Path;
use data::constants::{AppTitle, BackgroundColor, DefaultWindowSize,
	MinimumWindowSize};
use ::data::constants::{FileName_LogPrefix, Path_Logs};
use ::data::io::getConfigDir;
use freya::prelude::{LaunchConfig, WindowConfig, launch};
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use crate::components::LocalAchievementsApp;

fn main()
{
	let _guard = configureLogger();
	
	let tokioBuilder = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap();
	
	let _tokioRuntime = tokioBuilder.enter();
	
	launch(LaunchConfig::new()
		.with_window(
			WindowConfig::new_app(LocalAchievementsApp::new())
				.with_background(BackgroundColor)
				.with_min_size(MinimumWindowSize.0, MinimumWindowSize.1)
				.with_size(DefaultWindowSize.0, DefaultWindowSize.1)
				.with_title(AppTitle)
				.with_transparency(false)
		)
	);
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
