#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod data;
mod api;

pub use data::AuthObject as RetroAchievementsAuth;
pub use api::Api as RetroAchievementsApi;
