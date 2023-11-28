#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use ::dioxus::prelude::*;
use crate::{join, transmitMultiple};
use crate::background::{ApiCommand, SteamEndpoint};
use crate::data::Game;
use crate::io::{Path_Games, getImagePath};
use crate::platforms::steam::SteamApi;
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
pub fn Game(cx: Scope, game: Game) -> Element
{
	let showAchievements = use_state(cx, || false);
	
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
	
	let hasAchievements = !game.achievements.is_empty();
	let isHidden = match *showAchievements.get()
	{
		true => String::default(),
		false => "hidden".to_string(),
	};
	
	let headerClickHandler = move || {
		showAchievements.set(!showAchievements.get());
		
		if id > 0 && game.achievements.is_empty()
		{
			let commands = vec![
				ApiCommand::Steam(SteamEndpoint::SchemaForGame(id, SteamApi::Language_English.into())),
				ApiCommand::Steam(SteamEndpoint::PlayerAchievements(id, SteamApi::Language_English.into())),
			];
			transmitMultiple(commands);
		}
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "game",
			"appid": "{id}",
			
			if iconExists
			{
				rsx!(header
				{
					onclick: move |_| headerClickHandler(),
					
					img { class: "icon", alt: "Game Icon", src: "/{iconPath}", title: "{game.name}", },
					h3 { "{game.name}" }
				})
			}
			else
			{
				rsx!(header
				{
					onclick: move |_| headerClickHandler(),
					
					h3 { "{game.name}", }
				})
			}
			
			hasAchievements.then(|| rsx!{
				AchievementList { class: isHidden, game: game.clone() }
			})
		}
	});
}
