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
	let searchTerm = use_state(cx, || String::default());
	
	let mut games = userData.read().games.clone();
	games.sort_by(|a, b| a.partial_cmp(b).unwrap());
	
	let doRefresh = refresh.is_some_and(|switch| switch == true);
	
	return cx.render(rsx!
	{
		div
		{
			class: "gameList",
			id: "steamGameList",
			"refresh": doRefresh,
			
			div
			{
				class: "searchBar",
				
				label { r#for: "searchTerm", "Search:" }
				input
				{
					name: "searchTerm",
					placeholder: "Start typing to search by title",
					title: "Start typing to search by title",
					r#type: "text",
					oninput: move |e| searchTerm.set(e.value.to_owned()),
				}
			}
			
			for (i, game) in games.iter()
				.filter(|g| g.name.to_lowercase().contains(&searchTerm.get().to_owned().to_lowercase()))
				.enumerate()
			{
				Game { key: "{i}", game: game.clone(), refresh: doRefresh }
			}
		}
	});
}
