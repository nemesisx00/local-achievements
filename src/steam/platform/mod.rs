pub mod api;
pub mod data;

pub use data::{
	auth::AuthData,
	gameschema::{GameAchievement, Payload as Payload_GetSchemaForGame},
	globalpercentages::Payload as Payload_GetGlobalPercentages,
	ownedgames::{GameInfo, Payload as Payload_GetOwnedGames},
	playerachievements::{PlayerAchievement, Payload as Payload_GetPlayerAchievements},
	playersummaries::Payload as Payload_GetPlayerSummaries,
	recentlyplayedgames::Payload as Payload_GetRecentlyPlayedGames,
};
