use freya::hooks::{cow_borrowed, theme_with, ButtonThemeWith};
use freya::prelude::{
	component, dioxus_elements, fc_to_builder, rsx, Button, Element, GlobalSignal,
	Props, Readable, Signal, Writable
};
use crate::components::app::ActiveContent;
use crate::constants::{RetroAchievementsDarkBackground, SteamOrangeDarkBackground};

#[component]
pub fn NavBar(activeContent: Signal<ActiveContent>) -> Element
{
	return rsx!(
		rect
		{
			direction: "horizontal",
			main_align: "center",
			margin: "10",
			spacing: "5",
			width: "fill",
			
			Button
			{
				onclick: move |_| activeContent.set(ActiveContent::RetroAchievements),
				theme: theme_with!(ButtonTheme {
					hover_background: cow_borrowed!(RetroAchievementsDarkBackground),
					width: cow_borrowed!("175"),
				}),
				
				label { text_align: "center", "RetroAchievements" }
			}
			
			Button
			{
				onclick: move |_| activeContent.set(ActiveContent::Steam),
				theme: theme_with!(ButtonTheme {
					hover_background: cow_borrowed!(SteamOrangeDarkBackground),
					width: cow_borrowed!("100"),
				}),
				
				label { text_align: "center", "Steam" }
			}
			
			Button
			{
				onclick: move |_| activeContent.set(ActiveContent::Settings),
				theme: theme_with!(ButtonTheme {
					width: cow_borrowed!("100"),
				}),
				
				label { text_align: "center", "Settings" }
			}
		}
	);
}
