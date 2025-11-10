use freya::prelude::{component, dioxus_elements, fc_to_builder, rsx, spawn,
	use_hook, Element, GlobalSignal, IntoDynNode, Readable};
use crate::io::saveUserData_Steam;
use crate::{NotificationList, SteamAuthData, SteamUserData};
use crate::steam::platform::api::Api;
use super::game::GameElement;
use super::list::GameList;
use super::SelectedGameId;

#[component]
pub fn SteamContent() -> Element
{
	use_hook(|| if SteamUserData().id.is_empty()
	{
		refresh();
	});
	
	let selectedGame = match SelectedGameId()
	{
		None => None,
		Some(gameId) => SteamUserData().games.iter()
			.find(|g| g.id == gameId)
			.cloned(),
	};
	
	return rsx!(
		rect
		{
			cross_align: "center",
			direction: "vertical",
			spacing: "10",
			width: "fill",
			
			match selectedGame
			{
				None => rsx!(GameList {}),
				Some(game) => rsx!(GameElement { appId: game.id }),
			}
		}
	);
}

pub fn refresh()
{
	spawn(async move {
		let api = Api::from(SteamAuthData());
		loadUserProfile(&api).await;
		loadGameList(&api).await;
		//loadRecentlyPlayedGames(&api).await;
	});
}

async fn loadGameList(api: &Api)
{
	if let Ok(payload) = api.getOwnedGames().await
	{
		if !payload.response.games.is_empty()
		{
			SteamUserData.write().processOwnedGames(payload);
			match saveUserData_Steam(&SteamUserData())
			{
				Err(e) => println!("Error saving user data (Steam): {:?}", e),
				Ok(_) => println!("Saved user data (Steam)"),
			}
		}
		
		match api.cacheGameIcons(&SteamUserData().games, false).await
		{
			Some(games) => {
				let idList = games.iter()
					.fold(
						String::default(),
						|acc, game| format!("{}, {}", acc, game.id)
					);
				println!("SteamAPI: Error caching icon images for {}", idList[2..].to_string());
			},
			None => println!("SteamAPI: Icon images cached for owned games!"),
		}
	}
}

#[allow(unused)]
async fn loadRecentlyPlayedGames(api:&Api)
{
	if let Ok(_payload) = api.getRecentlyPlayedGames().await
	{
		//println!("{:?}", payload);
	}
}

async fn loadUserProfile(api: &Api)
{
	if let Ok(payload) = api.getPlayerSummaries().await
	{
		if let Some(profile) = payload.response.players.first()
		{
			NotificationList.write().push_back("Profile data downloaded".into());
			
			SteamUserData.write().update(
				&profile.steamid,
				&profile.personaname,
				match profile.avatarhash.is_empty()
				{
					true => None,
					false => Some(&profile.avatarhash)
				}
			);
			
			match saveUserData_Steam(&SteamUserData())
			{
				Err(e) => println!("Error saving user data (Steam): {:?}", e),
				Ok(_) => println!("Saved user data (Steam)"),
			}
			
			if !profile.avatarhash.is_empty()
			{
				match api.cacheProfileAvatar(&profile.steamid, &profile.avatarhash, false).await
				{
					Err(e) => {
						println!("Error caching avatar: {:?}", e);
						NotificationList.write().push_back("Error caching avatar".into());
					},
					
					Ok(_) => {
						println!("Avatar cached");
						NotificationList.write().push_back("Avatar icon cached".into());
					},
				}
			}
		}
	}
}
