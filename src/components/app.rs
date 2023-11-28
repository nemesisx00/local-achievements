#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use ::dioxus::prelude::*;
use crate::{receiver, transmit, userData};
use crate::background::{ApiCommand, CommandResponse, Internal};
use crate::hooks::useOnce;
use crate::io::{Path_Avatars, getImagePath};
use crate::platforms::steam::{SteamApi, SteamAuth};
use super::gamelist::GameList;
use super::steamdev::SteamDev;

/**
The root component of the application.
*/
pub fn App(cx: Scope) -> Element
{
	let internalRefresh = use_state(cx, || false);
	let steamAuth = use_state(cx, || SteamAuth::default());
	let steamActive = use_state(cx, || true);
	let settingsActive = use_state(cx, || false);
	
	useOnce(cx, || {
		to_owned![internalRefresh, steamAuth];
		cx.spawn(async move {
			startListeningToReceiver(&internalRefresh, &steamAuth).await;
		});
	});
	
	//Fetch the SteamAuth from the Dispatcher
	useOnce(cx, || transmit(ApiCommand::Metadata(Internal::GetSteamAuth)));
	
	let childRefresh = *internalRefresh.get();
	if childRefresh
	{
		internalRefresh.set(false);
		cx.needs_update();
	}
	
	let steamId = match userData().lock()
	{
		Ok(user) => user.steam.id.to_owned(),
		Err(_) => String::default(),
	};
	
	let avatar = match getImagePath(SteamApi::Platform.into(), Path_Avatars.into(), format!("{}_full.jpg", steamId))
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
	
	let mut games = match userData().lock()
	{
		Ok(user) => user.games.clone(),
		Err(_) => vec![],
	};
	games.sort_by(|a, b| a.partial_cmp(b).unwrap());
	
	return cx.render(rsx!
	{
		div { id: "fixedBackground" }
		
		nav
		{
			class: "navbar",
			
			div
			{
				class: "steam",
				onclick: move |_| {
					settingsActive.set(false);
					steamActive.set(true);
				},
				"Steam"
			}
			div
			{
				class: "settings",
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
				
				header
				{
					h1 { "Steam" }
					div
					{
						avatarExists.then(|| rsx!(img { alt: "Steam Avatar", src: "/{avatar}" }))
						
						SteamDev {}
					}
				}
				
				GameList { games: games.to_owned() }
			}
			
			section
			{
				class: "{settingsClass}",
				id: "settings",
				
				header
				{
					h1 { "Settings" }
				}
				
				div
				{
					h3 { "Update Steam Auth Info" }
					div
					{
						label { r#for: "authId", "Steam ID:" }
						input
						{
							name: "authId",
							r#type: "text",
							value: "{steamAuth.id}",
							onchange: move |e| transmit(ApiCommand::Metadata(Internal::UpdateSteamId(e.value.clone()))),
						}
					}
					div
					{
						label { r#for: "authApiKey", "API Key:" }
						input
						{
							name: "authApiKey",
							r#type: "text",
							value: "{steamAuth.key}",
							onchange: move |e| transmit(ApiCommand::Metadata(Internal::UpdateSteamApiKey(e.value.clone()))),
						}
					}
				}
			}
		}
	});
}

/**
Handle any communication originating from the dispatcher.
*/
async fn startListeningToReceiver(internalRefresh: &UseState<bool>, steamAuth: &UseState<SteamAuth>)
{
	if let Ok(mut channel) = receiver().lock()
	{
		if let Some(rx) = channel.as_mut()
		{
			loop
			{
				if let Some(response) = rx.recv().await
				{
					println!("Response received on the frontend: {:?}", response);
					match response
					{
						CommandResponse::Refresh => internalRefresh.set(true),
						CommandResponse::SteamAuth(auth) => steamAuth.set(auth.to_owned()),
					}
				}
			}
		}
	}
}
