#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::join;
use crate::data::Game;
use crate::io::{Path_Games, getImagePath};
use crate::platforms::steam::SteamApi;
use crate::state::{Steam, UserData};
use super::achievementlist::AchievementList;

/**
Component for displaying information about an individual game.

---

Property | Description
---|---
game | The game whose information is to be displayed.
first | Is this the first game in the list?
refresh | An optional boolean property used to force Dioxus to redraw the component.
*/
#[inline_props]
pub fn Game(cx: Scope, game: Game, first: bool, refresh: Option<bool>) -> Element
{
	let userData = use_atom_ref(cx, &UserData);
	let steam = use_atom_ref(cx, &Steam);
	
	let id = match &game.steam
	{
		Some(info) => info.id,
		None => 0,
	};
	
	let iconPath = match getImagePath(SteamApi::Platform.into(), join!(Path_Games, id), SteamApi::GameIcon.into())
	{
		Some(path) => path,
		None => String::new(),
	};
	
	let iconExists = !iconPath.is_empty() && Path::new(&iconPath).exists();
	let doRefresh = refresh.is_some_and(|switch| switch == true);
	
	let hasAchievements = !game.achievements.is_empty();
	
	return cx.render(rsx!
	{
		if !first
		{
			rsx!(hr {})
		}
		
		div
		{
			"appid": "{id}",
			"refresh": doRefresh,
			
			if iconExists
			{
				rsx!(h3 { img { src: "/{iconPath}", }, "{game.name}", })
			}
			else
			{
				rsx!(h3 { "{game.name}", })
			}
			
			(id > 0).then(|| rsx!{
				button
				{
					onclick: move |_| cx.spawn(
					{
						to_owned![steam, userData, id];
						async move {
							if let Ok(payload) = steam.read().getSchemaForGame(id, SteamApi::Language_English.into()).await
							{
								userData.write().processSteamAchievements(id, payload.getAchievements().to_owned());
							}
						}
					}),
					"Get Steam Achievements Schema"
				}
				
				button
				{
					onclick: move |_| cx.spawn(
					{
						to_owned![steam, id];
						async move {
							if let Ok(payload) = steam.read().getPlayerAchievements(id, SteamApi::Language_English.into()).await
							{
								println!("{:?}", payload);
							}
						}
					}),
					"Get Steam Achievements"
				}
			})
			
			hasAchievements.then(|| rsx!{
				AchievementList { game: game.clone(), refresh: doRefresh }
			})
		}
	});
}
