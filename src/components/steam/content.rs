use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder, rsx, spawn, use_hook, Button, Element, GlobalSignal, Readable};
use crate::components::steam::list::GameList;
use crate::io::{loadImageToBytes, saveUserData_Steam, Path_Avatars};
use crate::{SteamAuthData, SteamUserData};
use crate::platforms::steam::SteamApi;

#[component]
pub fn SteamContent() -> Element
{
	use_hook(|| if SteamUserData().id.is_empty()
	{
		refresh();
	});
	
	let avatar = loadIcon(
		&SteamApi::Platform.into(),
		&Path_Avatars.into(),
		&format!("{}_full.jpg", SteamUserData().id)
	);
	
	return rsx!(
		rect
		{
			cross_align: "center",
			direction: "vertical",
			spacing: "10",
			width: "fill",
			
			rect
			{
				direction: "horizontal",
				main_align: "center",
				spacing: "10",
				width: "50%",
				
				image { image_data: dynamic_bytes(avatar), width: "64", }
				
				rect
				{
					direction: "vertical",
					height: "100%",
					main_align: "space-around",
					
					label
					{
						main_align: "center",
						margin: "0 0 0 7",
						text_align: "center",
						
						"{SteamUserData().name}"
					}
					
					Button
					{
						onclick: move |_| refresh(),
						label { "Refresh" }
					}
				}
			}
			
			GameList {}
		}
	);
}

fn refresh()
{
	spawn(async move {
		let api = SteamApi::withAuth(SteamAuthData());
		loadUserProfile(&api).await;
		loadGameList(&api).await;
		//loadRecentlyPlayedGames(&api).await;
	});
}

async fn loadGameList(api: &SteamApi)
{
	if let Ok(payload) = api.getOwnedGames().await
	{
		if !payload.response.games.is_empty()
		{
			SteamUserData.write().processOwnedGames(payload);
			_ = saveUserData_Steam(&SteamUserData());
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

fn loadIcon<'a>(platform: &String, group: &String, fileName: &String) -> Vec<u8>
{
	return match loadImageToBytes(platform, group, fileName)
	{
		Ok(b) => b,
		Err(_) => vec![],
	};
}

#[allow(unused)]
async fn loadRecentlyPlayedGames(api:&SteamApi)
{
	if let Ok(_payload) = api.getRecentlyPlayedGames().await
	{
		//println!("{:?}", payload);
	}
}

async fn loadUserProfile(api: &SteamApi)
{
	if let Ok(payload) = api.getPlayerSummaries().await
	{
		if let Some(profile) = payload.response.players.first()
		{
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
					Ok(_) => println!("Avatar cached"),
					Err(e) => println!("Error caching avatar: {:?}", e)
				}
			}
		}
	}
}
