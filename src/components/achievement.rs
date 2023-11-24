#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use crate::{data::Achievement, platforms::Platform};

/**
Component for displaying information about an individual achievement.

---

Property | Description
---|---
achievement | The achievement whose information is to be displayed.
platform | (Optional) Restrict the displayed information to this platform.
refresh | (Optional) Force Dioxus to redraw the component.
*/
#[inline_props]
pub fn Achievement(cx: Scope, achievement: Achievement, _platform: Option<Platform>, refresh: Option<bool>) -> Element
{
	let mut names = vec![];
	for pi in &achievement.platforms
	{
		names.push(pi.name.to_owned());
	}
	let doRefresh = refresh.is_some_and(|switch| switch == true);
	
	return cx.render(rsx!
	{
		div
		{
			"refresh": doRefresh,
			
			for name in names.iter()
			{
				rsx!(p { "{name}" })
			}
		}
	});
}
