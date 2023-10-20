#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;

/**
The root component of the application.
*/
pub fn App(cx: Scope) -> Element
{
	return cx.render(rsx!(
		h1 { "Local Achievements" }
	));
}
