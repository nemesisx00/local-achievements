use freya::prelude::{component, dioxus_elements, fc_to_builder, rsx, spawn,
	use_hook, Button, Element, GlobalSignal, IntoDynNode, Readable};
use crate::{NotificationList, Rpcs3SettingsData, Rpcs3UserData};
use crate::io::saveUserData_Rpcs3;
use crate::rpcs3::platform::api::Api;
use super::SelectedGameId;
use super::game::GameElement;
use super::list::GameList;

#[component]
pub fn ContentElement() -> Element
{
	use_hook(|| if Rpcs3UserData().accountId != Rpcs3SettingsData().accountId
	{
		refresh();
	});
	
	let selectedGame = match SelectedGameId()
	{
		None => None,
		Some(npCommId) => Rpcs3UserData().games.iter()
			.cloned()
			.find(|g| g.npCommId == npCommId)
			.map(|g| g.npCommId),
	};
	
	return rsx!(
		rect
		{
			cross_align: "center",
			direction: "vertical",
			spacing: "10",
			width: "fill",
			
			Button
			{
				onpress: move |_| refresh(),
				label { "Refresh" }
			}
			
			match selectedGame
			{
				None => rsx!(GameList {}),
				Some(npCommId) => rsx!(GameElement { npCommId }),
			}
		}
	);
}

pub fn refresh()
{
	spawn(async move {
		let api: Api = Rpcs3SettingsData().into();
		
		match api.generateGameList()
		{
			Err(e) => println!("{:?}", e),
			Ok(games) => {
				for npCommId in games.iter().cloned().map(|g| g.npCommId)
				{
					if let Err(e) = api.cacheGameIcons(npCommId.to_owned())
					{
						println!("Error caching the icons for {}: {:?}", npCommId, e);
					}
				}
				NotificationList.write().push_back("Icons Cached".into());
				
				Rpcs3UserData.write().games = games;
				NotificationList.write().push_back("Trophy Data Loaded".into());
			}
		}
		
		Rpcs3UserData.write().accountId = Rpcs3SettingsData().accountId;
		
		match saveUserData_Rpcs3(&Rpcs3UserData())
		{
			Err(e) => println!("Error saving user data (RPCS3): {:?}", e),
			Ok(_) => println!("Saved user data (RPCS3)"),
		}
	});
}
