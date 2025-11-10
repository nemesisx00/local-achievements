use freya::prelude::{dioxus_elements, fc_to_builder, rsx, use_hook, Element,
	GlobalSignal, IntoDynNode, Readable, ThemeProvider};
use crate::{ActiveContent, NotificationList, RetroAchievementsAuthData,
	RetroAchievementsUserData, Rpcs3SettingsData, Rpcs3UserData, Settings,
	SteamAuthData, SteamUserData};
use crate::components::nav::NavBar;
use crate::components::notifications::NotificationElement;
use crate::components::settings::AppSettings;
use crate::constants::{BackgroundColor, TextColor, Theme};
use crate::io::{loadAppSettings, loadAuthData_RetroAchievements,
	loadAuthData_Steam, loadSettings_Rpcs3, loadUserData_RetroAchievements,
	loadUserData_Rpcs3, loadUserData_Steam};
use crate::retroachievements::RetroAchievementsContent;
use crate::rpcs3::Rpcs3ContentElement;
use crate::steam::SteamContent;

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
					ActiveContent::RetroAchievements => rsx!(RetroAchievementsContent {}),
					ActiveContent::Rpcs3 => rsx!(Rpcs3ContentElement {}),
					ActiveContent::Settings => rsx!(AppSettings {}),
					ActiveContent::Steam => rsx!(SteamContent {}),
				}
			}
		}
	);
}

fn initializeState()
{
	if let Ok(settings) = loadAppSettings()
	{
		*Settings.write() = settings;
	}
	
	if let Ok(rpcs3Settings) = loadSettings_Rpcs3()
	{
		*Rpcs3SettingsData.write() = rpcs3Settings;
	}
	
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
	
	if let Ok(user) = loadUserData_Rpcs3()
	{
		*Rpcs3UserData.write() = user;
	}
	
	if let Ok(user) = loadUserData_Steam()
	{
		*SteamUserData.write() = user;
	}
	
	NotificationList.write().push_back("Data loaded".into());
}
