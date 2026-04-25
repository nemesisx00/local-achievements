mod api;
mod endpoint;

pub use api::EgsApi;
pub use endpoint::{
	achievement::{Achievement, Payload_Achievement},
	player::Payload_PlayerProfile,
	private::{AchievementSummary, Payload_PlayerProfilePrivate},
	progress::{Payload_AchievementProgress, PlayerAchievementContainer},
};
