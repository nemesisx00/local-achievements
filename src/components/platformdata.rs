#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use chrono::LocalResult;
use ::chrono::prelude::*;
use ::dioxus::prelude::*;
use crate::{join, jpg, jpgAlt};
use crate::data::PlatformInfo;
use crate::io::{Path_Games, getImagePath};
use crate::platforms::{Icon_Locked, Platform};

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
pub fn PlatformData(cx: Scope, gameId: String, info: PlatformInfo) -> Element
{
	let name = info.name.to_owned();
	let description = info.description.to_owned();
	let id = info.id.to_owned();
	let globalPercentage = match info.globalPercentage
	{
		Some(gp) => gp.to_string(),
		None => String::default(),
	};
	
	let unlockTime = match info.timestamp
	{
		Some(ts) => {
			match NaiveDateTime::from_timestamp_millis(ts as i64)
			{
				Some(ndt) => match ndt.and_local_timezone(Local)
				{
					LocalResult::Single(dt) => dt.format("%B %d, %Y %l:%M %p")
						.to_string(),
					_ => String::default(),
				},
				None => String::default(),
			}
		},
		None => String::default(),
	};
	
	let filename = match info.timestamp.is_none()
	{
		true => jpgAlt!(info.id, Icon_Locked),
		false => jpg!(info.id),
	};
	let platform = Platform::nameOf(info.platform);
	let group = join!(Path_Games, gameId);
	
	let iconPath = match getImagePath(platform.to_owned(), group.to_owned(), filename)
	{
		Some(path) => path,
		None => String::default(),
	};
	
	let iconExists = !iconPath.is_empty() && Path::new(&iconPath).exists();
	
	return cx.render(rsx!
	{
		div
		{
			class: "platform",
			id: "{id}",
			
			div
			{
				class: "icon",
				
				iconExists.then(|| rsx!(img { alt: "Icon", src: "/{iconPath}" }))
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
					div { class: "right", "{unlockTime}" }
				}
			}
		}
	});
}
