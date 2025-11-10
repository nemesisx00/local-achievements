pub mod api;
pub mod data;

pub use data::{
	auth::AuthData,
	gameinfo::{AchievementMetadata, Payload as Payload_GetGameInfo},
	usercompletionprogress::{GameMetadata, Payload as Payload_GetUserCompletionProgress},
	userprofile::Payload as Payload_GetUserProfile,
};
