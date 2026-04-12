mod api;
mod auth;
mod endpoint;

pub use auth::RetroAchievementsAuth;
pub use api::RetroAchievementsApi;

pub use endpoint::{
	gameinfo::{AchievementMetadata, Payload_GetGameInfo},
	usercompletionprogress::{GameMetadata, Payload_GetUserCompletionProgress},
	userprofile::Payload_GetUserProfile,
};
