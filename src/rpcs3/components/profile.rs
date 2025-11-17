use dioxus::hooks::use_memo;
use freya::hooks::{cow_borrowed, theme_with};
use freya::prelude::{Button, Element, GlobalSignal, ProgressBar,
	ProgressBarThemeWith, Readable, component, dioxus_elements, fc_to_builder,
	rsx};
use crate::Rpcs3UserData;
use crate::constants::{RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorHardcore};
use super::content::refresh;

#[component]
pub fn UserProfile() -> Element
{
	use_memo(move || {
		Rpcs3UserData.write().calculatePoints();
	});
	
	let percent = (Rpcs3UserData().pointsToNextLevel() as f32
			/ Rpcs3UserData().pointTotalForLevel() as f32)
		* 100f32;
	
	let mut username = Rpcs3UserData().name;
	if username.is_empty()
	{
		username = Rpcs3UserData().formatAccountId();
	}
	
	return rsx!(
		rect
		{
			direction: "horizontal",
			main_align: "space-between",
			spacing: "25",
			width: "flex",
			
			rect
			{
				direction: "vertical",
				main_align: "space-around",
				spacing: "10",
				
				label
				{
					main_align: "center",
					text_align: "center",
					
					"{username}"
				}
				
				Button
				{
					onclick: move |_| refresh(),
					
					label
					{
						main_align: "center",
						text_align: "center",
						"Refresh"
					}
				}
			}
			
			rect
			{
				direction: "vertical",
				main_align: "space-around",
				spacing: "10",
				
				rect
				{
					content: "flex",
					cross_align: "center",
					direction: "horizontal",
					main_align: "start",
					spacing: "10",
					
					label { "Level {Rpcs3UserData().level()}" }
					
					ProgressBar
					{
						width: "flex",
						progress: percent,
						theme: theme_with!(ProgressBarTheme {
							background: cow_borrowed!(RetroAchievementsProgressColorBackground),
							height: cow_borrowed!("8"),
							progress_background: cow_borrowed!(RetroAchievementsProgressColorHardcore),
						}),
					}
				}
				
				label
				{
					main_align: "center",
					text_align: "center",
					width: "flex",
					
					"Platinums {Rpcs3UserData().platinumCount()}"
				}
			}
		}
	);
}
