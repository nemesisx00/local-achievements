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

use ::dioxus_desktop::launch;
use crate::components::App;

fn main()
{
	launch(App);
}
