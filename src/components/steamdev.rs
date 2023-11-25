#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::state::{UserData, Steam, saveUserData};

/**
Buttons for use during development. Will be phased out as the application
is filled out.
*/
pub fn SteamDev(cx: Scope) -> Element
{
	fermi::use_init_atom_root(cx);
	
	let userData = use_atom_ref(cx, &UserData);
	let steam = use_atom_ref(cx, &Steam);
	
	let devRefresh = use_shared_state::<bool>(cx).unwrap();
	let myRefresh = use_state(cx, || false);
	
	if *devRefresh.read() != *myRefresh.get()
	{
		*devRefresh.write() = *myRefresh.get();
	}
	
	return cx.render(rsx!
	{
		div
		{
			class: "steamDev",
			
			div
			{
				button
				{
					onclick: move |_| println!("Steam Info: {:?}", userData.read().steam),
					"Print Steam Info"
				}
				
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
				
				button
				{
					onclick: move |_| cx.spawn(
					{
						to_owned![steam, userData, myRefresh];
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
												myRefresh.set(!myRefresh.get());
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
						to_owned![steam, userData, myRefresh];
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
								
								myRefresh.set(!myRefresh.get());
							}
						}
					}),
					"Get Owned Games"
				}
				
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
				
				button
				{
					onclick: move |_| cx.spawn(
					{
						to_owned![steam];
						async move {
							if let Ok(payload) = steam.read().getRecentlyPlayedGames().await
							{
								println!("{:?}", payload);
							}
						}
					}),
					"Get Recently Played Games"
				}
			}
		}
	});
}
