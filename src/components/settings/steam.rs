use freya::hooks::{cow_borrowed, theme_with};
use freya::prelude::{component, fc_to_builder, rsx, use_signal, Element,
	GlobalSignal, Input, InputMode, Readable, Switch, SwitchThemeWith};
use freya::prelude::dioxus_elements::{self};
use crate::components::settings::{toggleInputMode, InputModeHiddenChar};
use crate::SteamAuthData;
use crate::io::saveAuthData_Steam;

#[component]
pub fn SteamSettings() -> Element
{
	let mut inputModeSteamApiKey = use_signal(|| InputMode::Hidden(InputModeHiddenChar));
	let mut inputModeSteamId = use_signal(|| InputMode::Hidden(InputModeHiddenChar));
	
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
					"Steam ID"
				}
				
				Input
				{
					mode: inputModeSteamId(),
					placeholder: "Steam ID",
					value: "{SteamAuthData().id}",
					width: "50%",
					onchange: move |value| {
						SteamAuthData.write().id = value;
						_ = saveAuthData_Steam(&SteamAuthData());
					},
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
					"Web API Key"
				}
				
				Input
				{
					mode: inputModeSteamApiKey(),
					placeholder: "Steam Web API Key",
					value: "{SteamAuthData().key}",
					width: "50%",
					onchange: move |value| {
						SteamAuthData.write().key = value;
						_ = saveAuthData_Steam(&SteamAuthData());
					},
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
