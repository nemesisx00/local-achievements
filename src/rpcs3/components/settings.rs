use dioxus::hooks::use_memo;
use freya::prelude::{component, fc_to_builder, rsx, use_signal, Element,
	GlobalSignal, Input, Readable, Props, Writable};
use freya::prelude::dioxus_elements::{self};
use crate::Rpcs3SettingsData;
use crate::components::NumericInput;
use crate::io::saveSettings_Rpcs3;

#[component]
pub fn SettingsElement(labelWidth: Option<String>) -> Element
{
	let labelWidth = match labelWidth
	{
		None => "20%".into(),
		Some(lw) => lw,
	};
	
	
	let accountId = use_signal(|| Rpcs3SettingsData().accountId);
	let mut appDataDir = use_signal(|| Rpcs3SettingsData().appDataDirectory);
	
	use_memo(move || {
		Rpcs3SettingsData.write().accountId = accountId();
		Rpcs3SettingsData.write().appDataDirectory = appDataDir();
		
		match saveSettings_Rpcs3(&Rpcs3SettingsData())
		{
			Err(e) => println!("Failed to save RPCS3 settings: {:?}", e),
			Ok(()) => println!("Saved RPCS3 settings"),
		}
	});
	
	return rsx!(
		rect
		{
			cross_align: "center",
			direction: "vertical",
			margin: "10",
			spacing: "5",
			width: "fill",
			
			label { margin: "0 0 5", text_align: "center", "RPCS3" }
			
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
					"Account ID"
				}
				
				NumericInput
				{
					placeholder: "RPCS3 Account ID",
					value: accountId,
					width: "flex",
				}
			}
			
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
					"App Data Directory"
				}
				
				Input
				{
					placeholder: "RPCS3 Application Data Directory",
					value: "{appDataDir()}",
					width: "flex",
					
					onchange: move |value| appDataDir.set(value),
				}
			}
		}
	);
}
