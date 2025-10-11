use freya::hooks::{cow_borrowed, theme_with, ButtonThemeWith};
use freya::prelude::{component, dioxus_elements, fc_to_builder, rsx, Button,
	Element, GlobalSignal, IntoDynNode, Readable};
use crate::ActiveContent;
use crate::components::retroachievements::RetroAchievementsUserProfile;
use crate::components::steam::SteamProfile;
use crate::constants::{RetroAchievementsDarkBackground, SteamOrangeDarkBackground};
use crate::SelectedGameId;

#[component]
pub fn NavBar() -> Element
{
	return rsx!(
		rect
		{
			direction: "horizontal",
			width: "fill",
			
			match ActiveContent()
			{
				ActiveContent::RetroAchievements => rsx!(RetroAchievementsUserProfile {}),
				ActiveContent::Steam => rsx!(SteamProfile {}),
				_ => rsx!(),
			}
			
			rect
			{
				direction: "horizontal",
				main_align: "center",
				margin: "10",
				spacing: "5",
				width: "fill",
				
				if SelectedGameId().is_some()
				{
					Button
					{
						onpress: move |_| *SelectedGameId.write() = None,
						label { "Back" }
					}
				}
				
				Button
				{
					onclick: move |_| *ActiveContent.write() = ActiveContent::RetroAchievements,
					theme: theme_with!(ButtonTheme {
						hover_background: cow_borrowed!(RetroAchievementsDarkBackground),
						width: cow_borrowed!("175"),
					}),
					
					label { text_align: "center", "RetroAchievements" }
				}
				
				Button
				{
					onclick: move |_| *ActiveContent.write() = ActiveContent::Steam,
					theme: theme_with!(ButtonTheme {
						hover_background: cow_borrowed!(SteamOrangeDarkBackground),
						width: cow_borrowed!("100"),
					}),
					
					label { text_align: "center", "Steam" }
				}
				
				Button
				{
					onclick: move |_| *ActiveContent.write() = ActiveContent::Settings,
					theme: theme_with!(ButtonTheme {
						width: cow_borrowed!("100"),
					}),
					
					label { text_align: "center", "Settings" }
				}
			}
		}
	);
}
