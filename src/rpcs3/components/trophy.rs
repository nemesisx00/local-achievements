use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, Element, GlobalSignal, IntoDynNode, Loader, Props, Readable};
use crate::io::{loadImageToBytes, Path_Games};
use crate::constants::BorderColor;
use crate::{join, Rpcs3UserData};
use crate::rpcs3::platform::api::Api;

#[component]
pub fn TrophyElement(npCommId: String, trophyId: u32) -> Element
{
	let game = match Rpcs3UserData().games.iter()
		.find(|g| g.npCommId == npCommId)
	{
		None => Default::default(),
		Some(g) => g.to_owned(),
	};
	
	let trophy = match game.trophies.iter()
		.find(|t| t.id == trophyId)
	{
		None => Default::default(),
		Some(trophy) => trophy.to_owned(),
	};
	
	let bytes = loadIcon(
		&Api::Platform.into(),
		&join!(Path_Games, npCommId),
		&format!(
			"{}{:03}.PNG",
			Api::TrophyIconPrefix,
			trophy.id
		)
	);
	
	let timestamp = match trophy.formatUnlockedTimestamp()
	{
		Err(_) => String::default(),
		Ok(ts) => ts,
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
				padding: "10 15",
				spacing: "10",
				width: "50%",
				
				rect
				{
					direction: "horizontal",
					min_height: "64",
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
						
						label { "{trophy.name}" }
						label { font_size: "10", max_height: "48", "{trophy.detail}" }
					}
				}
				
				rect
				{
					cross_align: "end",
					direction: "vertical",
					main_align: "space-between",
					min_width: "150",
					height: "100%",
					width: "150",
					
					label { font_size: "10", text_align: "center", width: "fill", "{timestamp}" }
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
