mod api;
mod endpoint;
mod starcraft2;

pub use api::BattleNetApi;
pub use starcraft2::Starcraft2;

pub use endpoint::{
	auth::BattleNetAuth,
	session::BattleNetSession,
	settings::BattleNetSettings,
	starcraft2::{
		account::PayloadPlayer,
		profile::{
			metadata::{AchievementMetadata, CategoryMetadata, PayloadStatic,
				RewardMetadata},
			profile::{CampaignDifficultyComplete, Career, EarnedAchievement,
				LeagueFinish, SnapshotSeason, SnapshotSeasonLeague,
				PayloadProfile, ProfileSnapshot, SwarmLevel},
		}
	},
	userinfo::UserInfo,
};
