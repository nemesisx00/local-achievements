/*!
Local Achievements is an open source desktop application for collecting, storing,
and tracking your achievements across multiple platforms in one unified UI.
*/

//Disable the additional command prompt window when running the application on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod battlenet;
mod components;
mod data;
//mod egs;
mod gog;
mod io;
mod macros;
mod net;
mod constants;
mod retroachievements;
mod rpcs3;
mod steam;
mod util;

use std::path::Path;
use std::sync::LazyLock;
use freya::prelude::{LaunchConfig, WindowConfig, launch};
use freya::radio::RadioStation;
use securestore::{KeySource, SecretsManager};
use tokio::sync::Mutex;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use crate::components::LocalAchievementsApp;
use crate::constants::{AppTitle, BackgroundColor, DefaultWindowSize,
	MinimumWindowSize};
use crate::data::AppData;
use crate::io::{FileName_LogPrefix, Path_Logs, getConfigDir, getSecretsKeyPath,
	getSecretsVaultPath};

static Secrets: LazyLock<Mutex<SecretsManager>> = LazyLock::new(|| {
	let keyPath = getSecretsKeyPath()
		.expect("Error getting the secrets key path");
	
	let vaultPath = getSecretsVaultPath()
		.expect("Error getting the secrets vault path");
	
	if !keyPath.exists() || !vaultPath.exists()
	{
		let m = SecretsManager::new(KeySource::Csprng)
			.expect("Error creating secrets manager with new key");
		
		_ = m.export_key(keyPath.clone())
			.expect("Error exporting new secrets key");
		
		_ = m.save_as(vaultPath.clone())
			.expect("Error saving new secrets vault to file");
	}
	
	Mutex::new(
		SecretsManager::load(vaultPath, KeySource::Path(&keyPath))
			.expect("Error loading secrets vault from file")
	)
});

fn main()
{
	let _guard = configureLogger();
	
	let tokioBuilder = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap();
	
	let _tokioRuntime = tokioBuilder.enter();
	
    let radioStation = RadioStation::create_global(AppData::default());
	
	launch(LaunchConfig::new()
		.with_window(
			WindowConfig::new_app(
					LocalAchievementsApp::new(radioStation)
				)
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
