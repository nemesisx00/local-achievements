use freya::prelude::{dioxus_elements, fc_to_builder, rsx, use_hook, Element,
	GlobalSignal, IntoDynNode, Readable, ThemeProvider};
use tracing::{info, info_span, warn};
use crate::{ActiveContent, NotificationList, RetroAchievementsAuthData,
	RetroAchievementsUserData, Rpcs3SettingsData, Rpcs3UserData, Settings,
	SteamAuthData, SteamUserData};
use crate::components::nav::NavBar;
use crate::components::notifications::NotificationElement;
use crate::components::profile::ProfileElement;
use crate::components::settings::AppSettings;
use crate::constants::{BackgroundColor, TextColor, Theme};
use crate::io::{loadAppSettings, loadAuthData_RetroAchievements,
	loadAuthData_Steam, loadSettings_Rpcs3, loadUserData_RetroAchievements,
	loadUserData_RetroAchievements_lossy, loadUserData_Rpcs3,
	loadUserData_Rpcs3_lossy, loadUserData_Steam, loadUserData_Steam_lossy};
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
				
				match ActiveContent()
				{
					ActiveContent::RetroAchievements => rsx!(RetroAchievementsContent {}),
					ActiveContent::Rpcs3 => rsx!(Rpcs3ContentElement {}),
					ActiveContent::Settings => rsx!(AppSettings {}),
					ActiveContent::Steam => rsx!(SteamContent {}),
				}
				
				ProfileElement {}
				NotificationElement {}
			}
		}
	);
}

fn initializeState()
{
	let span = info_span!("initializeState");
	let _guard = span.enter();
	
	if let Ok(settings) = loadAppSettings()
	{
		*Settings.write() = settings;
		info!("Application settings loaded!");
	}
	
	if let Ok(rpcs3Settings) = loadSettings_Rpcs3()
	{
		*Rpcs3SettingsData.write() = rpcs3Settings;
		info!("RPCS3 settings data loaded!");
	}
	
	if let Ok(retroAuth) = loadAuthData_RetroAchievements()
	{
		*RetroAchievementsAuthData.write() = retroAuth;
		info!("RetroAchievements auth data loaded!");
	}
	
	if let Ok(steamAuth) = loadAuthData_Steam()
	{
		*SteamAuthData.write() = steamAuth;
		info!("Steam auth data loaded!");
	}
	
	match loadUserData_RetroAchievements()
	{
		Err(e) => {
			warn!("Failed loading RetroAchievements user data: {:?}", e);
			match loadUserData_RetroAchievements_lossy()
			{
				Err(e) => warn!("Failed lossy loading RetroAchievements user data: {:?}", e),
				Ok(user) => {
					*RetroAchievementsUserData.write() = user;
					info!("RetroAchievements user data loaded - lossy method!");
				},
			}
		},
		
		Ok(user) => {
			*RetroAchievementsUserData.write() = user;
			info!("RetroAchievements user data loaded!");
		},
	}
	
	match loadUserData_Rpcs3()
	{
		Err(e) => {
			warn!("Failed loading RPCS3 user data: {:?}", e);
			match loadUserData_Rpcs3_lossy()
			{
				Err(e) => warn!("Failed lossy loading RPCS3 user data: {:?}", e),
				Ok(user) => {
					*Rpcs3UserData.write() = user;
					info!("RPCS3 user data loaded - lossy method!");
				},
			}
		},
		
		Ok(user) => {
			*Rpcs3UserData.write() = user;
			info!("RPCS3 user data loaded!");
		},
	}
	
	match loadUserData_Steam()
	{
		Err(e) => {
			warn!("Failed loading Steam user data: {:?}", e);
			match loadUserData_Steam_lossy()
			{
				Err(e) => warn!("Failed lossy loading Steam user data: {:?}", e),
				Ok(user) => {
					*SteamUserData.write() = user;
					info!("Steam user data loaded - lossy method!");
				},
			}
		},
		
		Ok(user) => {
			*SteamUserData.write() = user;
			info!("Steam user data loaded!");
		},
	}
	
	NotificationList.write().push_back("Data loaded".into());
	info!("Data loaded!");
}
