use freya::prelude::{component, fc_to_builder, rsx, Element, GlobalSignal, Input,
	IntoDynNode, Readable, Props};
use freya::prelude::dioxus_elements::{self};
use crate::io::{getCacheDir, getConfigDir, getDataDir};

#[component]
pub fn LocalInfo(labelWidth: Option<String>) -> Element
{
	let labelWidth = match labelWidth
	{
		None => "20%".into(),
		Some(lw) => lw,
	};
	
	let cacheDir = getCacheDir(false);
	let configDir = getConfigDir(false);
	let dataDir = getDataDir(false);
	
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
					content: "flex",
					direction: "horizontal",
					main_align: "center",
					spacing: "5",
					width: "75%",
					
					label
					{
						margin: "5 5 0 0",
						min_width: "102",
						text_align: "end",
						width: "{labelWidth}",
						"Cache"
					}
					
					Input
					{
						placeholder: "",
						value: "{dir}",
						width: "flex",
						onchange: move |_| {},
					}
				}
			}
			
			if let Some(dir) = configDir
			{
				rect
				{
					content: "flex",
					direction: "horizontal",
					main_align: "center",
					spacing: "5",
					width: "75%",
					
					label
					{
						margin: "5 5 0 0",
						min_width: "102",
						text_align: "end",
						width: "{labelWidth}",
						"Configuration"
					}
					
					Input
					{
						placeholder: "",
						value: "{dir}",
						width: "flex",
						onchange: move |_| {},
					}
				}
			}
			
			if let Some(dir) = dataDir
			{
				rect
				{
					content: "flex",
					direction: "horizontal",
					main_align: "center",
					spacing: "5",
					width: "75%",
					
					label
					{
						margin: "5 5 0 0",
						min_width: "102",
						text_align: "end",
						width: "{labelWidth}",
						"Data"
					}
					
					Input
					{
						placeholder: "",
						value: "{dir}",
						width: "flex",
						onchange: move |_| {},
					}
				}
			}
		}
	);
}
