use freya::prelude::{dioxus_elements, fc_to_builder, rsx, use_hook, Element,
	GlobalSignal, IntoDynNode, Readable, ThemeProvider};
use crate::components::nav::NavBar;
use crate::components::notifications::NotificationElement;
use crate::components::retroachievements::RetroAchivementsContent;
use crate::components::settings::AppSettings;
use crate::components::steam::SteamContent;
use crate::io::{loadAuthData_RetroAchievements, loadAuthData_Steam, loadUserData_RetroAchievements, loadUserData_Steam};
use crate::constants::{BackgroundColor, TextColor, Theme};
use crate::{ActiveContent, NotificationList, RetroAchievementsAuthData, RetroAchievementsUserData, SteamAuthData, SteamUserData};

pub fn App() -> Element
{
	use_hook(initializeState);
	
	return rsx!(
		ThemeProvider
		{
			theme: Theme,
			
			rect
			{
				background: BackgroundColor,
				color: TextColor,
				cross_align: "center",
				height: "100%",
				padding: "15",
				
				NavBar {}
				
				NotificationElement {}
				
				match ActiveContent()
				{
					ActiveContent::RetroAchievements => rsx!(RetroAchivementsContent {}),
					ActiveContent::Settings => rsx!(AppSettings {}),
					ActiveContent::Steam => rsx!(SteamContent {}),
				}
			}
		}
	);
}

fn initializeState()
{
	if let Ok(retroAuth) = loadAuthData_RetroAchievements()
	{
		*RetroAchievementsAuthData.write() = retroAuth;
	}
	
	if let Ok(steamAuth) = loadAuthData_Steam()
	{
		*SteamAuthData.write() = steamAuth;
	}
	
	if let Ok(user) = loadUserData_RetroAchievements()
	{
		*RetroAchievementsUserData.write() = user;
	}
	
	if let Ok(user) = loadUserData_Steam()
	{
		*SteamUserData.write() = user;
	}
	
	NotificationList.write().push_back("Data loaded".into());
}
