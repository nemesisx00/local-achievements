use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder, rsx, Element, GlobalSignal, IntoDynNode, Loader, Props, Readable};
use crate::data::{RetroAchievementsAchievement, RetroAchievementsMode};
use crate::io::{loadImageToBytes, Path_Games};
use crate::constants::{BorderColor, Icon_Locked};
use crate::platforms::retroachievements::RetroAchievementsApi;
use crate::{join, png, pngAlt, RetroAchievementsUserData};

#[component]
pub fn AchievementElement(gameId: usize, achievementId: usize) -> Element
{
	let achievement = match RetroAchievementsUserData().games.iter()
		.find(|g| g.id == gameId)
	{
		None => RetroAchievementsAchievement::default(),
		Some(game) => match game.achievements.iter()
			.find(|a| a.id == achievementId)
		{
			None => RetroAchievementsAchievement::default(),
			Some(achievement) => achievement.to_owned(),
		},
	};
	
	let unlockedCasual = achievement.unlocked(RetroAchievementsMode::Casual);
	let unlockedHardcore = achievement.unlocked(RetroAchievementsMode::Hardcore);
	
	let iconName = match unlockedCasual || unlockedHardcore
	{
		true => png!(achievement.name),
		false => pngAlt!(achievement.name, Icon_Locked),
	};
	
	let bytes = loadIcon(
		&RetroAchievementsApi::Platform.to_lowercase(),
		&join!(Path_Games, gameId),
		&iconName
	);
	
	let timestamp = match achievement.formatEarnedTimestamp(match unlockedHardcore {
			false => RetroAchievementsMode::Casual,
			true => RetroAchievementsMode::Hardcore,
		})
	{
		Err(_) => String::default(),
		Ok(ts) => ts,
	};
	
	/*
	let globalPercentage = match achievement.globalPercentage
	{
		Some(gp) => format!("{}% of players have this achievement", gp),
		None => String::default(),
	};
	*/
	let globalPercentage = "% of players have this achievements".to_string();
	
	return rsx!(
		rect
		{
			border: "1 center {BorderColor}",
			corner_radius: "10",
			direction: "horizontal",
			margin: "5",
			min_height: "64",
			padding: "5 10",
			spacing: "10",
			width: "fill",
			
			match bytes.is_empty()
			{
				true => rsx!(Loader{}),
				false => rsx!(image
				{
					image_data: dynamic_bytes(bytes),
					width: "64",
				})
			}
			
			rect
			{
				direction: "vertical",
				height: "100%",
				main_align: "space-between",
				spacing: "15",
				width: "60%",
				
				label { "{achievement.name}" }
				label { font_size: "10", max_height: "48", "{achievement.description}" }
			}
			
			rect
			{
				cross_align: "end",
				direction: "vertical",
				main_align: "space-between",
				min_width: "150",
				height: "100%",
				width: "calc(100%-60%-64-10-10)",
				
				label { font_size: "10", text_align: "end", "{globalPercentage}"}
				label { font_size: "10", text_align: "end", "{timestamp}" }
			}
		}
	);
}

fn loadIcon<'a>(platform: &String, group: &String, fileName: &String) -> Vec<u8>
{
	return match loadImageToBytes(platform, group, fileName)
	{
		Ok(b) => b,
		Err(_) => vec![],
	};
}
