use freya::prelude::{component, dioxus_elements, fc_to_builder, rsx, Element,
	GlobalSignal, Readable};
use crate::components::settings::local::LocalInfo;
use crate::components::settings::notifications::NotificationSettings;
use crate::retroachievements::RetroAchievementsSettingsElement;
use crate::rpcs3::Rpcs3SettingsElement;
use crate::steam::SteamSettingsElement;

#[component]
pub fn AppSettings() -> Element
{
	return rsx!(
		rect
		{
			direction: "vertical",
			width: "fill",
			
			NotificationSettings {}
			RetroAchievementsSettingsElement {}
			Rpcs3SettingsElement {}
			SteamSettingsElement {}
			LocalInfo {}
		}
	);
}
