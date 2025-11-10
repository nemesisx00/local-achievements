use freya::prelude::{component, fc_to_builder, rsx, use_memo, use_signal,
	Element, GlobalSignal, Readable, Props};
use freya::prelude::dioxus_elements::{self};
use crate::Settings;
use crate::components::numput::NumericInput;
use crate::io::saveAppSettings;

#[component]
pub fn NotificationSettings(labelWidth: Option<String>) -> Element
{
	let labelWidth = match labelWidth
	{
		None => "20%".into(),
		Some(lw) => lw,
	};
	
	let duration = use_signal(|| Settings().notificationDuration);
	
	use_memo(move || {
		Settings.write().notificationDuration = duration();
		
		match saveAppSettings(&Settings())
		{
			Err(e) => println!("Failed to save application settings: {:?}", e),
			Ok(()) => println!("Saved application settings"),
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
			
			label
			{
				margin: "0 0 5",
				text_align: "center",
				"Notifications"
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
					"Duration"
				}
				
				NumericInput
				{
					min: 500,
					max: 5000,
					placeholder: "1000",
					value: duration,
					width: "flex",
				}
			}
		}
	);
}
