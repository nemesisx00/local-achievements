use freya::prelude::{component, fc_to_builder, rsx, Element, GlobalSignal, Input,
	IntoDynNode, Readable};
use freya::prelude::dioxus_elements::{self};
use crate::io::{getCacheDir, getConfigDir, getDataDir};

#[component]
pub fn LocalInfo() -> Element
{
	let cacheDir = getCacheDir(false);
	let configDir = getConfigDir(false);
	let dataDir = getDataDir(false);
	
	let inputWidth = "61%";
	
	return rsx!(
		rect
		{
			cross_align: "center",
			direction: "vertical",
			margin: "10",
			spacing: "5",
			width: "fill",
			
			label
			{
				margin: "0 0 5",
				text_align: "center",
				"Local Directories"
			}
			
			if let Some(dir) = cacheDir
			{
				rect
				{
					direction: "horizontal",
					main_align: "center",
					spacing: "5",
					width: "75%",
					
					label
					{
						margin: "5 5 0 0",
						min_width: "102",
						text_align: "end",
						width: "15%",
						"Cache"
					}
					
					Input
					{
						placeholder: "",
						value: "{dir}",
						width: "{inputWidth}",
						onchange: move |_| {},
					}
				}
			}
			
			if let Some(dir) = configDir
			{
				rect
				{
					direction: "horizontal",
					main_align: "center",
					spacing: "5",
					width: "75%",
					
					label
					{
						margin: "5 5 0 0",
						min_width: "102",
						text_align: "end",
						width: "15%",
						"Configuration"
					}
					
					Input
					{
						placeholder: "",
						value: "{dir}",
						width: "{inputWidth}",
						onchange: move |_| {},
					}
				}
			}
			
			if let Some(dir) = dataDir
			{
				rect
				{
					direction: "horizontal",
					main_align: "center",
					spacing: "5",
					width: "75%",
					
					label
					{
						margin: "5 5 0 0",
						min_width: "102",
						text_align: "end",
						width: "15%",
						"Data"
					}
					
					Input
					{
						placeholder: "",
						value: "{dir}",
						width: "{inputWidth}",
						onchange: move |_| {},
					}
				}
			}
		}
	);
}
