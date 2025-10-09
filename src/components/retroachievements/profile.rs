use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder, rsx, with_owner, Button, Element, EventHandler, GlobalSignal, Owner, Props, Readable};
use crate::{png, RetroAchievementsUserData};
use crate::io::{loadImageToBytes, Path_Avatars};
use crate::platforms::retroachievements::RetroAchievementsApi;

#[component]
pub fn RetroAchivementsUserProfile(refreshHandler: Option<EventHandler>) -> Element
{
	let avatar = match RetroAchievementsUserData().ulid
	{
		None => vec![],
		Some(ulid) => loadIcon(
			&RetroAchievementsApi::Platform.into(),
			&Path_Avatars.into(),
			&png!(ulid)
		),
	};
	
	return rsx!(	
		rect
		{
			direction: "horizontal",
			main_align: "center",
			spacing: "10",
			width: "50%",
			
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
					onclick: refreshHandler,
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
