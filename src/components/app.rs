#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::platforms::steam::SteamApi;
use crate::state::{User, RetroAchievementsAuth, SteamAuth, loadUserData, saveUserData};

/**
The root component of the application.
*/
pub fn App(cx: Scope) -> Element
{
	fermi::use_init_atom_root(cx);
	
	let _retroAuth = use_atom_ref(cx, &RetroAchievementsAuth);
	let steamAuth = use_atom_ref(cx, &SteamAuth);
	let user = use_atom_ref(cx, &User);
	
	let api = use_ref(cx, || SteamApi::new(steamAuth.read().clone()).unwrap());
	let id = use_state(cx, || steamAuth.read().id.to_owned());
	let apiKey = use_state(cx, || steamAuth.read().key.to_owned());
	
	let api1 = api.clone();
	let user1 = user.clone();
	let playerSummaries = use_future(cx, (), |_| async move
	{
		println!("api1 auth: {:?}", api1.read().auth);
		println!("api1 validate? {}", api1.read().auth.validate());
		
		match api1.read().getPlayerSummaries().await
		{
			Ok(payload) => {
				println!("{:?}", payload);
				match payload.response.players.first()
				{
					Some(profile) => {
						let mut userRef = user1.write();
						userRef.steam.name = profile.personaname.clone();
						userRef.steam.id = profile.steamid.clone();
					},
					None => {},
				}
			},
			Err(e) => println!("Failed to retrieve player summaries! {:?}", e),
		}
	});
	
	let api2 = api.clone();
	let user2 = user.clone();
	let ownedGames = use_future(cx, (), |_| async move
	{
		println!("api2 auth: {:?}", api2.read().auth);
		println!("api2 validate? {}", api2.read().auth.validate());
		
		match api2.read().getOwnedGames().await
		{
			Ok(payload) => {
				println!("game count: {}", payload.response.game_count);
				user2.write().processSteamGames(payload.response.games.clone());
			},
			Err(e) => println!("Failed to retrieve owned games! {:?}", e),
		}
	});
	
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
					let mut steamRef = steamAuth.write();
					steamRef.id = id.to_string();
					steamRef.key = apiKey.to_string();
					api.write().auth = steamRef.clone();
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
						onclick: move |_| println!("Steam Info: {:?}", user.read().steam),
						"Print Steam Info"
					}
				}
				div
				{
					button
					{
						onclick: move |_| {
							match loadUserData()
							{
								Ok(data) => {
									println!("Profile loaded!");
									*user.write() = data;
								},
								Err(_) => println!("Failed to load profile!"),
							}
						},
						"Load Profile Data"
					}
				}
				div
				{
					button
					{
						onclick: move |_| {
							match saveUserData(user.read().clone())
							{
								Ok(_) => println!("Profile saved!"),
								Err(_) => println!("Failed to save profile!"),
							}
						},
						"Save Profile Data"
					}
				}
			}
			
			div
			{
				button
				{
					onclick: move |_|
					{
						if api.read().auth.validate()
						{
							playerSummaries.cancel(cx);
							playerSummaries.restart();
						}
					},
					"Get Player Summaries"
				}
			}
			
			div
			{
				button
				{
					onclick: move |_| {
						if api.read().auth.validate()
						{
							ownedGames.cancel(cx);
							ownedGames.restart();
						}
					},
					"Get Owned Games"
				}
			}
		}
	});
}
