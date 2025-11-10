use dioxus::hooks::use_memo;
use freya::hooks::{cow_borrowed, theme_with};
use freya::prelude::{component, fc_to_builder, rsx, use_signal, Element,
	GlobalSignal, Input, InputMode, Readable, Props, Switch, SwitchThemeWith};
use freya::prelude::dioxus_elements::{self};
use crate::components::{InputModeHiddenChar, toggleInputMode};
use crate::RetroAchievementsAuthData;
use crate::io::saveAuthData_RetroAchievements;

#[component]
pub fn SettingsElement(labelWidth: Option<String>) -> Element
{
	let labelWidth = match labelWidth
	{
		None => "20%".into(),
		Some(lw) => lw,
	};
	
	let mut inputModeApiKey = use_signal(|| InputMode::Hidden(InputModeHiddenChar));
	let mut inputModeUsername = use_signal(|| InputMode::Hidden(InputModeHiddenChar));
	
	use_memo(move || {
		match saveAuthData_RetroAchievements(&RetroAchievementsAuthData())
		{
			Err(e) => println!("Failed to save RetroAchievements authorization data: {:?}", e),
			Ok(()) => println!("Saved RetroAchievements authorization data"),
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
			
			label { margin: "0 0 5", text_align: "center", "RetroAchievements Web API Authentication" }
			
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
					"Username"
				}
				
				Input
				{
					mode: inputModeUsername(),
					placeholder: "RetroAchievements Username",
					value: "{RetroAchievementsAuthData().username}",
					width: "flex",
					
					onchange: move |value| RetroAchievementsAuthData.write().username = value,
				}
				
				label { margin: "5 0 0 5", text_align: "end", "Show" }
				
				Switch
				{
					theme: theme_with!(SwitchTheme {
						margin: cow_borrowed!("4 0 0"),
					}),
					enabled: inputModeUsername() == InputMode::Shown,
					ontoggled: move |_| toggleInputMode(&mut inputModeUsername)
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
					"Web API Key"
				}
				
				Input
				{
					mode: inputModeApiKey(),
					placeholder: "RetroAchievements Web API Key",
					value: "{RetroAchievementsAuthData().key}",
					width: "flex",
					
					onchange: move |value| RetroAchievementsAuthData.write().key = value,
				}
				
				label { margin: "5 0 0 5", text_align: "end", "Show" }
				
				Switch
				{
					theme: theme_with!(SwitchTheme {
						margin: cow_borrowed!("4 0 0"),
					}),
					enabled: inputModeApiKey() == InputMode::Shown,
					ontoggled: move |_| toggleInputMode(&mut inputModeApiKey)
				}
			}
		}
	);
}
