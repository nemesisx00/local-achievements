use freya::prelude::{dioxus_elements, fc_to_builder, rsx, use_hook, use_signal,
	Element, GlobalSignal, IntoDynNode, Readable, ThemeProvider};
use crate::components::nav::NavBar;
use crate::components::retroachievements::RetroAchivementsContent;
use crate::components::settings::AppSettings;
use crate::components::steam::SteamContent;
use crate::io::{loadAuthData_RetroAchievements, loadAuthData_Steam, loadUserData_RetroAchievements, loadUserData_Steam};
use crate::constants::{BackgroundColor, TextColor, Theme};
use crate::{RetroAchievementsAuthData, RetroAchievementsUserData, SteamAuthData, SteamUserData};

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub enum ActiveContent
{
	#[default]
	RetroAchievements,
	Settings,
	Steam,
}

pub fn App() -> Element
{
	use_hook(initializeState);
	
	let activeContent = use_signal(|| ActiveContent::default());
	
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
				
				NavBar { activeContent }
				
				match activeContent()
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
}
