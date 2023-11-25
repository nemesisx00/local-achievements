#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::io::{Path_Avatars, getImagePath};
use crate::platforms::steam::SteamApi;
use crate::state::{UserData, RetroAchievements, Steam};
use super::gamelist::GameList;
use super::steamdev::SteamDev;

/**
The root component of the application.
*/
pub fn App(cx: Scope) -> Element
{
	fermi::use_init_atom_root(cx);
	
	let userData = use_atom_ref(cx, &UserData);
	let _retroA = use_atom_ref(cx, &RetroAchievements);
	let steam = use_atom_ref(cx, &Steam);
	
	let id = use_state(cx, || steam.read().auth.id.clone());
	let apiKey = use_state(cx, || steam.read().auth.key.clone());
	let internalRefresh = use_state(cx, || false);
	
	let steamActive = use_state(cx, || true);
	let settingsActive = use_state(cx, || false);
	
	use_shared_state_provider(cx, || false);
	let devRefresh = use_shared_state::<bool>(cx).unwrap();
	
	let avatar = match getImagePath(SteamApi::Platform.into(), Path_Avatars.into(), format!("{}_full.jpg", userData.read().steam.id))
	{
		Some(path) => path,
		None => String::new(),
	};
	
	let avatarExists = !avatar.is_empty() && Path::new(&avatar).exists();
	
	let steamClass = match steamActive.get()
	{
		true => "active",
		false => "inactive",
	};
	
	let settingsClass = match settingsActive.get()
	{
		true => "active",
		false => "inactive",
	};
	
	return cx.render(rsx!
	{
		h1 { "Local Achievements" }
		
		nav
		{
			class: "navbar",
			
			div
			{
				onclick: move |_| {
					settingsActive.set(false);
					steamActive.set(true);
				},
				"Steam"
			}
			div
			{
				onclick: move |_| {
					steamActive.set(false);
					settingsActive.set(true);
				},
				"Settings"
			}
		}
		
		div
		{
			class: "content",
			
			section
			{
				class: "{steamClass}",
				id: "steam",
				
				h3 { "Steam" }
				div
				{
					avatarExists.then(|| rsx!(img { alt: "Steam Avatar", src: "/{avatar}" }))
				}
				SteamDev {}
				GameList { refresh: *devRefresh.read() || *internalRefresh.get() }
			}
			
			section
			{
				class: "{settingsClass}",
				id: "settings",
				
				div
				{
					h3 { "Update Steam Auth Info" }
					div
					{
						label { r#for: "authId", "Steam ID:" }
						input { name: "authId", r#type: "text", value: "{id}", onchange: move |e| id.set(e.value.clone()) }
					}
					div
					{
						label { r#for: "authApiKey", "API Key:" }
						input { name: "authApiKey", r#type: "text", value: "{apiKey}", onchange: move |e| apiKey.set(e.value.clone()) }
					}
					button
					{
						onclick: move |_| {
							let mut steamRef = steam.write();
							steamRef.auth.id = id.to_string();
							steamRef.auth.key = apiKey.to_string();
						},
						"Update"
					}
				}
			}
		}
	});
}
