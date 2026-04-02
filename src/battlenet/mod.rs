mod components;
mod data;
mod platform;

pub use components::{
	content::BattleNetContentElement,
	profile::BattleNetUserProfile,
	settings::SettingsElement as BattleNetSettingsElement,
};

pub use data::{
	games::Games as BattleNetGames,
	user::User as BattleNetUser,
};

pub use platform::data::{
	auth::{BattleNetAuth, BattleNetSettings},
	session::BattleNetSession,
};
