use freya::hooks::{cow_borrowed, theme_with};
use freya::prelude::{component, fc_to_builder, rsx, use_memo, use_signal,
	Element, GlobalSignal, Input, InputMode, Readable, Props, Switch,
	SwitchThemeWith};
use freya::prelude::dioxus_elements::{self};
use crate::components::{InputModeHiddenChar, toggleInputMode};
use crate::SteamAuthData;
use crate::io::saveAuthData_Steam;

#[component]
pub fn SettingsElement(labelWidth: Option<String>) -> Element
{
	let labelWidth = match labelWidth
	{
		None => "20%".into(),
		Some(lw) => lw,
	};
	
	let mut inputModeSteamApiKey = use_signal(|| InputMode::Hidden(InputModeHiddenChar));
	let mut inputModeSteamId = use_signal(|| InputMode::Hidden(InputModeHiddenChar));
	
	use_memo(move || {
		match saveAuthData_Steam(&SteamAuthData())
		{
			Err(e) => println!("Failed to save Steam authorization data: {:?}", e),
			Ok(()) => println!("Saved Steam authorization data"),
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
			
			label { margin: "0 0 5", text_align: "center", "Steam Web API Authentication" }
			
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
					"Steam ID"
				}
				
				Input
				{
					mode: inputModeSteamId(),
					placeholder: "Steam ID",
					value: "{SteamAuthData().id}",
					width: "flex",
					
					onchange: move |value| SteamAuthData.write().id = value,
				}
				
				label { margin: "5 0 0 5", text_align: "end", "Show" }
				
				Switch
				{
					theme: theme_with!(SwitchTheme {
						margin: cow_borrowed!("4 0 0"),
					}),
					enabled: inputModeSteamId() == InputMode::Shown,
					ontoggled: move |_| toggleInputMode(&mut inputModeSteamId)
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
					mode: inputModeSteamApiKey(),
					placeholder: "Steam Web API Key",
					value: "{SteamAuthData().key}",
					width: "flex",
					
					onchange: move |value| SteamAuthData.write().key = value,
				}
				
				label { margin: "5 0 0 5", text_align: "end", "Show" }
				
				Switch
				{
					theme: theme_with!(SwitchTheme {
						margin: cow_borrowed!("4 0 0"),
					}),
					enabled: inputModeSteamApiKey() == InputMode::Shown,
					ontoggled: move |_| toggleInputMode(&mut inputModeSteamApiKey)
				}
			}
		}
	);
}
