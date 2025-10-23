use freya::hooks::{cow_borrowed, ButtonTheme, FontTheme, Theme, DARK_THEME};

pub const AppTitle: &str = "Local Achievements";
#[allow(unused)]
pub const AppVersion: &str = "0.3.0";
pub const DefaultWindowSize: (f64, f64) = (1280.0, 720.0);
pub const MinimumWindowSize: (f64, f64) = (720.0, 480.0);

pub const CornerRadius: u32 = 5;

pub const BackgroundColor: &str = "rgb(35, 35, 35)";
pub const ButtonBackgroundColor: &str = "rgb(26, 26, 26)";
pub const ButtonHoverColor: &str = "rgb(48, 48, 48)";
pub const BorderColor: &str = "rgb(78, 78, 78)";
pub const TextColor: &str = "rgb(204, 204, 204)";
pub const TransparentColor: &str = "transparent";

pub const RetroAchievementsDarkBackground: &str = "rgb(5, 60, 135)";
pub const RetroAchievementsProgressColorBackground: &str = "rgb(9, 9, 11)";
pub const RetroAchievementsProgressColorCasual: &str = "rgb(115, 115, 115)";
pub const RetroAchievementsProgressColorHardcore: &str = "rgb(250, 186, 6)";

#[allow(unused)]
pub const SteamContrast: &str = "rgb(31, 98, 154)";
#[allow(unused)]
pub const SteamContrastDark: &str = "rgb(6, 57, 99)";
#[allow(unused)]
pub const SteamOrange: &str = "rgb(238, 94, 34)";
#[allow(unused)]
pub const SteamOrangeDark: &str = "rgb(219, 83, 27)";
pub const SteamOrangeDarkBackground: &str = "rgb(134, 40, 0)";

pub const Format_ChronoDateTime: &str = "%B %d, %Y %l:%M %p";

pub const Icon_Locked: &str = "locked";

pub const TheString: &str = "The ";

pub const Theme: Theme = Theme
{
	button: ButtonTheme
	{
		background: cow_borrowed!(ButtonBackgroundColor),
		border_fill: cow_borrowed!(BorderColor),
		corner_radius: cow_borrowed!("10"),
		focus_border_fill: cow_borrowed!(BorderColor),
		
		font_theme: FontTheme
		{
			color: cow_borrowed!(TextColor),
		},
		
		hover_background: cow_borrowed!(ButtonHoverColor),
		
		..DARK_THEME.button
	},
	
	..DARK_THEME
};
