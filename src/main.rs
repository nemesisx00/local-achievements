/*!
Local Achievements is an open source desktop application for collecting, storing,
and tracking your achievements across multiple platforms in one unified UI.
*/

#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod components;
mod data;
mod io;
mod macros;
mod platforms;
mod state;

use ::dioxus_desktop::{launch_with_props, Config};
use crate::components::{App, AppProps};
use crate::io::{readAuth_RetroAchievements, readAuth_Steam};

fn main()
{
	let retroAuth = match readAuth_RetroAchievements()
	{
		Ok(auth) => Some(auth),
		Err(_) => None,
	};
	
	let steamAuth = match readAuth_Steam()
	{
		Ok(auth) => Some(auth),
		Err(_) => None,
	};
	
	launch_with_props(App, AppProps { retroAuth, steamAuth }, Config::default());
}
