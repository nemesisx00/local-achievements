use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, Button, Element, GlobalSignal, Readable};
use crate::components::steam::content::refresh;
use crate::io::{loadImageToBytes, Path_Avatars};
use crate::SteamUserData;
use crate::platforms::steam::SteamApi;

#[component]
pub fn SteamProfile() -> Element
{
	let avatar = loadIcon(
		&SteamApi::Platform.into(),
		&Path_Avatars.into(),
		&format!("{}_full.jpg", SteamUserData().id)
	);
	
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
					
					"{SteamUserData().name}"
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
