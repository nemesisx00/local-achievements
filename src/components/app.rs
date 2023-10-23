#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::state::{UserData, RetroAchievements, Steam};

/**
The root component of the application.
*/
pub fn App(cx: Scope) -> Element
{
	fermi::use_init_atom_root(cx);
	
	let _retroA = use_atom_ref(cx, &RetroAchievements);
	let steam = use_atom_ref(cx, &Steam);
	let userData = use_atom_ref(cx, &UserData);
	
	let id = use_state(cx, || String::new());
	let apiKey = use_state(cx, || String::new());
	
	return cx.render(rsx!
	{
		h1 { "Local Achievements" }
		
		hr {}
		
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
		
		hr {}
		
		div
		{
			h3 { "Steam" }
			
			div
			{
				div
				{
					button
					{
						onclick: move |_| println!("Steam Info: {:?}", userData.read().steam),
						"Print Steam Info"
					}
				}
			}
			
			div
			{
				button
				{
					onclick: move |_| cx.spawn(
					{
						to_owned![steam, userData];
						async move {
							if let Ok(payload) = steam.read().getPlayerSummaries().await
							{
								println!("{:?}", payload);
								if let Some(profile) = payload.response.players.first()
								{
									userData.write().steam.update(
										profile.steamid.to_owned(),
										profile.personaname.to_owned(),
										match profile.avatarhash.is_empty()
										{
											true => None,
											false => Some(profile.avatarhash.to_owned()),
										}
									);
								}
							}
						}
					}),
					"Get Player Summaries"
				}
			}
			
			div
			{
				button
				{
					onclick: move |_| cx.spawn(
					{
						to_owned![steam, userData];
						async move {
							if let Ok(payload) = steam.read().getOwnedGames().await
							{
								println!("Game count: {}", payload.response.game_count);
								userData.write().processSteamGames(payload.response.games);
							}
						}
					}),
					"Get Owned Games"
				}
			}
		}
	});
}
