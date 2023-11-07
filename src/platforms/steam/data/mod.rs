#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod auth;
mod getgameschema;
mod getglobalpercentages;
mod getownedgames;
mod getplayerachievements;
mod getplayersummaries;

pub use auth::AuthData;
pub use getgameschema::GetSchemaForGamePayload;
pub use getglobalpercentages::GetGlobalPercentagesPayload;
pub use getownedgames::{GameInfo, GetOwnedGamesPayload};
pub use getplayerachievements::GetPlayerAchievementsPayload;
pub use getplayersummaries::GetPlayerSummariesPayload;
