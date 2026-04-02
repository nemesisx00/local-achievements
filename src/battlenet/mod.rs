mod components;
mod data;
mod platform;

pub use components::{
	content::BattleNetContentElement,
	profile::BattleNetUserProfile,
	refresh::handleDataOperation,
	settings::SettingsElement as BattleNetSettingsElement,
};

pub use data::{
	games::Games as BattleNetGames,
	user::User as BattleNetUser,
};

pub use platform::data::{
	auth::BattleNetAuth,
	session::BattleNetSession,
	settings::BattleNetSettings,
};
