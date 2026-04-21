use freya::prelude::{Color, CornerRadius};

pub const AppTitle: &str = "Reliquarian";
#[allow(unused)]
pub const AppVersion: &str = "0.5.0";
pub const DefaultWindowSize: (f64, f64) = (1280.0, 720.0);
pub const MinimumWindowSize: (f64, f64) = (720.0, 480.0);

pub const BackgroundColor: Color = Color::from_rgb(35, 35, 35);
pub const ButtonBackgroundColor: Color = Color::from_rgb(26, 26, 26);
pub const ButtonHoverColor: Color = Color::from_rgb(48, 48, 48);
pub const BorderColor: Color = Color::from_rgb(78, 78, 78);

pub const CornerRadius: CornerRadius = CornerRadius::new_all(5.0);

pub const DefaultHttpRequestRate: u64 = 50;

pub const FileName_GameHeader: &str = "game-header";
pub const FileName_GameIcon: &str = "game-icon";
pub const FileName_LogPrefix: &str = "app.log";

pub const Format_ChronoDateTime: &str = "%B %d, %Y %l:%M %p";

pub const GogProgressColor: Color = Color::from_rgb(13, 186, 132);

pub const Icon_Locked: &str = "locked";

pub const InputModeHiddenChar: char = '*';

pub const LinkBlue: Color = Color::from_rgb(173, 194, 252);

pub const OverlayBackgroundColor: Color = Color::from_rgb(12, 12, 12);
pub const OverlayGreyoutColor: Color = Color::from_argb(128, 0, 0, 0);

pub const Path_Avatars: &str = "avatars";
pub const Path_Logs: &str = "logs";
pub const Path_Games: &str = "games";

pub const RetroAchievementsProgressColorBackground: Color = Color::from_rgb(9, 9, 11);
pub const RetroAchievementsProgressColorCasual: Color = Color::from_rgb(115, 115, 115);
pub const RetroAchievementsProgressColorHardcore: Color = Color::from_rgb(250, 186, 6);

pub const SecretsKeyFileName: &str = "secrets.key";
pub const SecretsVaultFileName: &str = "secrets.json";

pub const SteamContrast: Color = Color::from_rgb(31, 98, 154);

pub const TextColor: Color = Color::from_rgb(204, 204, 204);
pub const TheString: &str = "The ";
