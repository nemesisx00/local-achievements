#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::cmp::Ordering;

use ::dioxus::prelude::*;
use crate::data::Game;
use super::achievement::Achievement;

/**
Component for displaying the list of games.

---

Property | Description
---|---
refresh | An optional boolean property used to force Dioxus to redraw the component.
*/
#[inline_props]
pub fn AchievementList(cx: Scope, class: Option<String>, game: Game, refresh: Option<bool>) -> Element
{
	let mut achievements = game.achievements.clone();
	achievements.sort_by(|a, b| {
		let mut response = Ordering::Equal;
		for ad in &a.platforms
		{
			if let Some(bd) = b.platforms.iter().find(|d| d.platform == ad.platform)
			{
				response = match bd.timestamp
				{
					Some(_) => match ad.timestamp
					{
						Some(_) => ad.name.partial_cmp(&bd.name).unwrap(),
						None => Ordering::Greater,
					},
					None => match ad.timestamp
					{
						Some(_) => Ordering::Less,
						None => ad.name.partial_cmp(&bd.name).unwrap(),
					}
				};
			}
		}
		response
	});
	
	let className = format!("achievementList {}", class.clone().unwrap_or_default());
	let doRefresh = refresh.is_some_and(|switch| switch == true);
	
	return cx.render(rsx!
	{
		div
		{
			class: "{className.trim()}",
			"refresh": doRefresh,
			
			for (i, achievement) in achievements.iter().enumerate()
			{
				Achievement { key: "{i}", gameIds: game.getIds(), achievement: achievement.clone(), refresh: doRefresh }
			}
		}
	});
}
