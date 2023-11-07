#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod api;
mod data;

pub use data::{AuthData as SteamAuth, GameInfo as SteamGame};
pub use api::Api as SteamApi;
