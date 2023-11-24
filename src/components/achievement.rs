#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use crate::data::Achievement;
use crate::platforms::Platform;
use super::platformdata::PlatformData;

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
pub fn Achievement(cx: Scope, achievement: Achievement, platform: Option<Platform>, refresh: Option<bool>) -> Element
{
	let doRefresh = refresh.is_some_and(|switch| switch == true);
	
	let mut data = vec![];
	
	match platform
	{
		Some(pl) => {
			match achievement.platforms.iter().find(|p| pl == &p.platform)
			{
				Some(info) => data.push(rsx!(PlatformData { info: info.to_owned(), refresh: doRefresh })),
				None => data.push(rsx!(div { "No info found" }))
			}
		},
		None => {
			for info in &achievement.platforms
			{
				data.push(rsx!(PlatformData { key: "{info.id.to_owned()}", info: info.to_owned(), refresh: doRefresh }))
			}
		}
	}
	
	return cx.render(rsx!
	{
		div
		{
			class: "achievement",
			"refresh": doRefresh,
			
			for node in data
			{
				node
			}
		}
	});
}
