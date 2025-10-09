mod auth;
mod gameinfo;
mod usercompletionprogress;
mod userprofile;

pub use auth::AuthData as RetroAchievementsAuth;
pub use gameinfo::{AchievementMetadata, Payload as Payload_GetGameInfo};
pub use usercompletionprogress::{GameMetadata, Payload as Payload_GetUserCompletionProgress};
pub use userprofile::Payload as Payload_GetUserProfile;
