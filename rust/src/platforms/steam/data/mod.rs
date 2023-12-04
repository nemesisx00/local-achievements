mod auth;
mod gameschema;
mod globalpercentages;
mod ownedgames;
mod playerachievements;
mod playersummaries;
mod recentlyplayedgames;

pub use auth::AuthData;
pub use gameschema::{GameAchievement, GetSchemaForGamePayload};
pub use globalpercentages::GetGlobalPercentagesPayload;
pub use ownedgames::{GameInfo, GetOwnedGamesPayload};
pub use playerachievements::{PlayerAchievement, GetPlayerAchievementsPayload};
pub use playersummaries::GetPlayerSummariesPayload;
pub use recentlyplayedgames::GetRecentlyPlayedGamesPayload;
