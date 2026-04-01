use freya::prelude::{ButtonLayoutThemePreference, Color, ColorsSheet,
	CornerRadius, Preference, ProgressBarThemePreference, Theme, DARK_THEME};

pub const AppTitle: &str = "Local Achievements";
#[allow(unused)]
pub const AppVersion: &str = "0.4.0";
pub const DefaultWindowSize: (f64, f64) = (1280.0, 720.0);
pub const MinimumWindowSize: (f64, f64) = (720.0, 480.0);

pub const SecretsKeyFileName: &str = "secrets.key";
pub const SecretsVaultFileName: &str = "secrets.json";

pub const DefaultHttpRequestRate: u64 = 50;

pub const CornerRadius: CornerRadius = CornerRadius::new_all(5.0);

pub const BackgroundColor: Color = Color::from_rgb(35, 35, 35);
pub const ButtonBackgroundColor: Color = Color::from_rgb(26, 26, 26);
pub const ButtonHoverColor: Color = Color::from_rgb(48, 48, 48);
pub const BorderColor: Color = Color::from_rgb(78, 78, 78);
pub const OverlayBackgroundColor: Color = Color::from_rgb(12, 12, 12);
pub const OverlayGreyoutColor: Color = Color::from_argb(128, 0, 0, 0);
pub const TextColor: Color = Color::from_rgb(204, 204, 204);

#[allow(unused)]
pub const RetroAchievementsDarkBackground: Color = Color::from_rgb(5, 60, 135);
pub const RetroAchievementsProgressColorBackground: Color = Color::from_rgb(9, 9, 11);
pub const RetroAchievementsProgressColorCasual: Color = Color::from_rgb(115, 115, 115);
pub const RetroAchievementsProgressColorHardcore: Color = Color::from_rgb(250, 186, 6);

#[allow(unused)]
pub const SteamContrast: Color = Color::from_rgb(31, 98, 154);
#[allow(unused)]
pub const SteamContrastDark: Color = Color::from_rgb(6, 57, 99);
#[allow(unused)]
pub const SteamOrange: Color = Color::from_rgb(238, 94, 34);
#[allow(unused)]
pub const SteamOrangeDark: Color = Color::from_rgb(219, 83, 27);
#[allow(unused)]
pub const SteamOrangeDarkBackground: Color = Color::from_rgb(134, 40, 0);

pub const Format_ChronoDateTime: &str = "%B %d, %Y %l:%M %p";

pub const Icon_Locked: &str = "locked";

pub const TheString: &str = "The ";

pub const AppTheme: Theme = Theme
{
	name: "local-achievements",
	button_layout: ButtonLayoutThemePreference
	{
		corner_radius: Preference::Specific(CornerRadius),
		..DARK_THEME.button_layout
	},
	colors: ColorsSheet
	{
		background: ButtonBackgroundColor,
		border: BorderColor,
		border_focus: BorderColor,
		hover: ButtonHoverColor,
		text_primary: TextColor,
		..DARK_THEME.colors
	},
	progressbar:  ProgressBarThemePreference
	{
		background: Preference::Specific(RetroAchievementsProgressColorBackground),
		color: Preference::Specific(RetroAchievementsProgressColorCasual),
		height: Preference::Specific(12.0),
		progress_background: Preference::Specific(RetroAchievementsProgressColorCasual),
		..DARK_THEME.progressbar
	},
	..DARK_THEME
};
