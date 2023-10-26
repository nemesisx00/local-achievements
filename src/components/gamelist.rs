#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::data::Game;
use crate::io::{Path_Games, getImagePath};
use crate::platforms::steam::SteamApi;
use crate::state::UserData;

pub fn GameList(cx: Scope) -> Element
{
	let userData = use_atom_ref(cx, &UserData);
	
	let mut games = userData.read().games.clone();
	games.sort_by(|a, b| a.partial_cmp(b).unwrap());
	
	return cx.render(rsx!
	{
		for game in games.iter()
		{
			Game { game: game.clone(), first: games.first().is_some_and(|f| f == game), hasAchievements: game.achievements.len() > 0 }
		}
	});
}

#[inline_props]
pub fn Game(cx: Scope, game: Game, first: bool, hasAchievements: bool) -> Element
{
	let id = match &game.steam
	{
		Some(info) => info.id,
		None => 0,
	};
	
	let iconPath = match getImagePath(SteamApi::Platform.into(), Path_Games.into(), SteamApi::iconFileName(id))
	{
		Some(path) => path,
		None => String::new(),
	};
	
	return cx.render(rsx!
	{
		if !first
		{
			rsx!(hr {})
		}
		
		div
		{
			h3 { img { src: "/{iconPath}", }, "{game.name}", }
			hasAchievements.then(|| rsx!{
				p { "Has achievements" }
			})
		}
	});
}
