#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::io::{Path_Avatars, getImagePath};
use crate::platforms::steam::SteamApi;
use crate::state::{UserData, RetroAchievements, Steam, saveUserData};
use super::gamelist::GameList;

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
	let steamRefresh = use_state(cx, || false);
	
	let avatar = match getImagePath(SteamApi::Platform.into(), Path_Avatars.into(), format!("{}_full.jpg", userData.read().steam.id))
	{
		Some(path) => path,
		None => String::new(),
	};
	
	let avatarExists = !avatar.is_empty() && Path::new(&avatar).exists();
	
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
				avatarExists.then(|| rsx!(img { alt: "Steam Avatar", src: "/{avatar}" }))
			}
			
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
					onclick: move |_| {
						match saveUserData(userData.read().clone())
						{
							Ok(_) => println!("User data saved!"),
							Err(e) => println!("Error saving user data: {:?}", e),
						}
					},
					"Save Data"
				}
			}
			
			div
			{
				button
				{
					onclick: move |_| cx.spawn(
					{
						to_owned![steam, userData, steamRefresh];
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
									
									if let Some(hash) = &userData.read().steam.avatar
									{
										match steam.read().cacheProfileAvatar(userData.read().steam.id.to_owned(), hash.to_owned(), false).await
										{
											Ok(_) => {
												println!("Avatars cached");
												steamRefresh.set(!steamRefresh.get());
											},
											Err(e) => println!("Error caching avatars: {:?}", e),
										}
									}
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
						to_owned![steam, userData, steamRefresh];
						async move {
							if let Ok(payload) = steam.read().getOwnedGames().await
							{
								println!("Game count: {}", payload.response.game_count);
								userData.write().processSteamGames(payload.response.games);
								let failed = steam.read().cacheGameIcons(userData.read().getAllSteamInfo(), false).await;
								
								match failed
								{
									None => println!("SteamApi: Icon images cached for owned games!"),
									Some(games) => {
										let mut idList = String::new();
										games.iter().for_each(|game| idList = format!("{}, {}", idList, game.id));
										println!("SteamApi: Error caching icon images for {}", idList[2..].to_string());
									}
								}
								
								steamRefresh.set(!steamRefresh.get());
							}
						}
					}),
					"Get Owned Games"
				}
			}
			
			div
			{
				button
				{
					onclick: move |_| cx.spawn(
					{
						to_owned![steam];
						async move {
							let tekken7 = 389730;
							if let Ok(payload) = steam.read().getGlobalPercentages(tekken7).await
							{
								println!("{:?}", payload.asMap());
							}
						}
					}),
					"Get Global Percentages"
				}
			}
			
			GameList { refresh: steamRefresh.get().to_owned() }
		}
	});
}
