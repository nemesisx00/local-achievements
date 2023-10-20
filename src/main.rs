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
use crate::io::readAuth_RetroAchievements;

fn main()
{
	let auth = readAuth_RetroAchievements().unwrap();
	launch_with_props(App, AppProps { auth: Some(auth.to_owned()) }, Config::default());
}
