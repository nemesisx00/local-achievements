#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use crate::data::PlatformInfo;
use crate::platforms::Platform;

/**
Component for displaying information about an individual achievement from an
individual platform.

---

Property | Description
---|---
achievement | The achievement whose information is to be displayed.
platform | (Optional) Restrict the displayed information to this platform.
refresh | (Optional) Force Dioxus to redraw the component.
*/
#[inline_props]
pub fn PlatformData(cx: Scope, info: PlatformInfo, refresh: Option<bool>) -> Element
{
	let name = info.name.to_owned();
	let description = info.description.to_owned();
	let id = info.id.to_owned();
	let globalPercentage = match info.globalPercentage
	{
		Some(gp) => gp.to_string(),
		None => String::default(),
	};
	let platform = Platform::nameOf(info.platform);
	
	let doRefresh = refresh.is_some_and(|switch| switch == true);
	
	return cx.render(rsx!
	{
		div
		{
			class: "platform",
			id: "{id}",
			"refresh": doRefresh,
			
			div
			{
				class: "icon",
				
				img { alt: "Icon" }
			}
			div
			{
				class: "data",
				
				div
				{
					class: "row",
					h4 { class: "left", "{name}" }
					h4 { class: "right", "{globalPercentage}" }
				}
				
				div
				{
					class: "row",
					
					div { class: "left", "{description}" }
					div { class: "right", "{platform}" }
				}
			}
		}
	});
}
