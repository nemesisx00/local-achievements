#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::state::UserData;
use super::game::Game;

/**
Component for displaying the list of games.

---

Property | Description
---|---
refresh | An optional boolean property used to force Dioxus to redraw the component.
*/
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
				Game { game: game.clone(), first: games.first().is_some_and(|f| f == game), refresh: doRefresh }
			}
		}
	});
}
