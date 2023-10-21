/*!
Local Achievements is an open source desktop application for collecting, storing,
and tracking your achievements across multiple platforms in one unified UI.
*/

#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod components;
mod platforms;
mod io;

use ::dioxus_desktop::{launch_with_props, Config};
use crate::components::{App, AppProps};
use crate::io::{readAuth_RetroAchievements, readAuth_Steam};

fn main()
{
	let retro = readAuth_RetroAchievements().unwrap();
	let steam = readAuth_Steam().unwrap();
	launch_with_props(App, AppProps { retroAuth: Some(retro.to_owned()), steamAuth: Some(steam.to_owned()) }, Config::default());
}
