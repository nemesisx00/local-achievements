use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder, rsx, Element, GlobalSignal, IntoDynNode, Loader, Props, Readable};
use crate::data::SteamAchievement;
use crate::io::{loadImageToBytes, Path_Games};
use crate::platforms::steam::SteamApi;
use crate::constants::{BorderColor, Icon_Locked};
use crate::{join, jpg, jpgAlt, SteamUserData};

#[component]
pub fn AchievementElement(appId: usize, id: String) -> Element
{
	let achievement = match SteamUserData().games.iter()
		.find(|g| g.id == appId)
	{
		None => SteamAchievement::default(),
		Some(game) => match game.achievements.iter()
			.find(|a| a.id == id)
		{
			None => SteamAchievement::default(),
			Some(a) => a.clone(),
		}
	};
	
	let iconName = match achievement.unlocked()
	{
		true => jpg!(achievement.id),
		false => jpgAlt!(achievement.id, Icon_Locked),
	};
	
	let bytes = loadIcon(
		&SteamApi::Platform.to_lowercase(),
		&join!(Path_Games, appId),
		&iconName
	);
	
	let timestamp = match achievement.formatTimestamp()
	{
		None => String::default(),
		Some(ts) => ts,
	};
	
	let globalPercentage = match achievement.globalPercentage
	{
		Some(gp) => format!("{}% of players have this achievement", gp),
		None => String::default(),
	};
	
	return rsx!(
		rect
		{
			direction: "horizontal",
			main_align: "space-around",
			margin: "5 0",
			width: "fill",
			
			rect
			{
				border: "1 center {BorderColor}",
				corner_radius: "10",
				direction: "horizontal",
				main_align: "space-between",
				margin: "5",
				min_height: "64",
				min_width: "540",
				padding: "5 10",
				width: "50%",
				
				rect
				{
					direction: "horizontal",
					spacing: "10",
					
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
						min_height: "64",
						max_height: "256",
						spacing: "15",
						width: "60%",
						
						label { "{achievement.name}" }
						label { font_size: "10", "{achievement.description}" }
					}
				}
				
				rect
				{
					cross_align: "end",
					direction: "vertical",
					height: "100%",
					main_align: "space-between",
					width: "200",
					
					label { font_size: "10", text_align: "end", "{globalPercentage}"}
					label { font_size: "10", text_align: "end", "{timestamp}" }
				}
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
