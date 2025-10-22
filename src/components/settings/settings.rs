use freya::prelude::{component, dioxus_elements, fc_to_builder, rsx, Element, GlobalSignal, Readable};
use crate::components::settings::local::LocalInfo;
use crate::components::settings::retroachievements::RetroAchievementsSettings;
use crate::components::settings::steam::SteamSettings;

#[component]
pub fn AppSettings() -> Element
{
	return rsx!(
		rect
		{
			direction: "vertical",
			width: "fill",
			
			RetroAchievementsSettings {}
			SteamSettings {}
			LocalInfo {}
		}
	);
}
