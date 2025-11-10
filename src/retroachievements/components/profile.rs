use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, Button, Element, GlobalSignal, Readable};
use crate::{png, RetroAchievementsUserData};
use crate::io::{loadImageToBytes, Path_Avatars};
use crate::retroachievements::platform::api::Api;
use super::content::refresh;

#[component]
pub fn RetroAchievementsUserProfile() -> Element
{
	let avatar = match RetroAchievementsUserData().ulid
	{
		None => vec![],
		Some(ulid) => loadIcon(
			&Api::Platform.into(),
			&Path_Avatars.into(),
			&png!(ulid)
		),
	};
	
	return rsx!(
		rect
		{
			position: "absolute",
			position_left: "0",
			position_top: "0",
			direction: "horizontal",
			main_align: "start",
			spacing: "10",
			
			image { image_data: dynamic_bytes(avatar), width: "64", }
			
			rect
			{
				direction: "vertical",
				height: "100%",
				main_align: "space-around",
				
				label
				{
					main_align: "center",
					margin: "0 0 0 7",
					text_align: "center",
					
					"{RetroAchievementsUserData().username}"
				}
				
				Button
				{
					onclick: move |_| refresh(),
					label { "Refresh" }
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
