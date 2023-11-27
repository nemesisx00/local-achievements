#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod game;
mod retroachievements;
mod steam;

pub use game::Game;
pub use steam::SteamInfo;
