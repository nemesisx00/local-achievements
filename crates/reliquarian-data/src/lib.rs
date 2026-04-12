pub mod constants;
pub mod enums;
pub mod filter;
pub mod format;
pub mod io;
pub mod settings;
pub mod state;

use std::sync::LazyLock;
use freya::prelude::{ButtonLayoutThemePreference, ColorsSheet, Gaps, Preference,
	ProgressBarThemePreference, Size, Theme, dark_theme};
use securestore::{KeySource, SecretsManager};
use tokio::sync::Mutex;
use crate::constants::{BorderColor, ButtonBackgroundColor, ButtonHoverColor,
	CornerRadius, RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorCasual, TextColor};
use crate::io::{getSecretsKeyPath, getSecretsVaultPath};

pub static Secrets: LazyLock<Mutex<SecretsManager>> = LazyLock::new(|| {
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

pub fn localAchievementsTheme() -> Theme {
    let mut theme = dark_theme();
	
    theme.name = "localAchievements";
	
	theme.colors = ColorsSheet
	{
		background: ButtonBackgroundColor,
		border: BorderColor,
		border_focus: BorderColor,
		hover: ButtonHoverColor,
		text_primary: TextColor,
		..dark_theme().colors
	};
	
	theme.set(
		"button_layout",
		ButtonLayoutThemePreference
		{
			corner_radius: Preference::Specific(CornerRadius),
			height: Preference::Specific(Size::auto()),
			margin: Preference::Specific(Gaps::new_all(0.0)),
			padding: Preference::Specific(Gaps::new_symmetric(5.0, 10.0)),
			width: Preference::Specific(Size::auto()),
		}
	);
	
	theme.set(
		"progressbar",
		ProgressBarThemePreference
		{
			background: Preference::Specific(RetroAchievementsProgressColorBackground),
			color: Preference::Specific(RetroAchievementsProgressColorCasual),
			height: Preference::Specific(10.0),
			progress_background: Preference::Specific(RetroAchievementsProgressColorCasual),
		}
	);
	
    return theme;
}
