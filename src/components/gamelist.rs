#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::join;
use crate::data::Game;
use crate::io::{Path_Games, getImagePath};
use crate::platforms::steam::SteamApi;
use crate::state::UserData;

#[inline_props]
pub fn GameList(cx: Scope, refresh: Option<bool>) -> Element
{
	let userData = use_atom_ref(cx, &UserData);
	
	let mut games = userData.read().games.clone();
	games.sort_by(|a, b| a.partial_cmp(b).unwrap());
	
	let doRefresh = refresh.is_some_and(|switch| switch == true);
	
	return cx.render(rsx!
	{
		div
		{
			id: "steamGameList",
			"refresh": doRefresh,
			
			for game in games.iter()
			{
				Game { game: game.clone(), first: games.first().is_some_and(|f| f == game), hasAchievements: game.achievements.len() > 0, refresh: doRefresh }
			}
		}
	});
}

#[inline_props]
pub fn Game(cx: Scope, game: Game, first: bool, hasAchievements: bool, refresh: Option<bool>) -> Element
{
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
			
			hasAchievements.then(|| rsx!{
				p { "Has achievements" }
			})
		}
	});
}
