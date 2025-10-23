use freya::events::Code;
use freya::hooks::{cow_borrowed, theme_with};
use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, spawn, use_hook, use_scroll_controller, use_signal, Button,
	ButtonThemeWith, Element, GlobalSignal, Input, IntoDynNode, Loader, Props,
	Readable, ScrollConfig, ScrollDirection, ScrollPosition, VirtualScrollView,
	Writable};
use crate::components::steam::achievement::AchievementElement;
use crate::data::{SteamAchievement, SteamGame};
use crate::io::{loadImageToBytes, saveUserData_Steam, Filename_GameIcon, Path_Games};
use crate::{join, jpg, Language, NotificationList, SteamAuthData, SteamUserData};
use crate::platforms::steam::SteamApi;

#[component]
pub fn GameElement(appId: usize) -> Element
{
	let mut scrollController = use_scroll_controller(|| ScrollConfig::default());
	let mut search = use_signal(|| String::default());
	
	let game = match SteamUserData().games.iter()
		.find(|g| g.id == appId)
	{
		None => SteamGame::default(),
		Some(g) => g.clone(),
	};
	
	let mut achievementsList: Vec<SteamAchievement> = game.achievements.iter()
		.cloned()
		.filter(|a| a.name.to_lowercase().contains(&search().to_lowercase())
			|| a.description.to_lowercase().contains(&search().to_lowercase()))
		.collect();
	achievementsList.sort();
	
	let bytes = loadIcon(&game);
	
	use_hook(|| if !game.loaded && appId > 0
	{
		refresh(appId);
	});
	
	return rsx!(
		if !game.loaded
		{
			rect
			{
				direction: "horizontal",
				main_align: "center",
				width: "fill",
				Loader {}
			}
		}
		else
		{
			rect
			{
				direction: "vertical",
				cross_align: "center",
				margin: "10 0 5",
				spacing: "10",
				width: "fill",
				
				onglobalkeyup: move |e| match e.code
				{
					Code::Home => scrollController.scroll_to(ScrollPosition::Start, ScrollDirection::Vertical),
					Code::End => scrollController.scroll_to(ScrollPosition::End, ScrollDirection::Vertical),
					_ => {},
				},
				
				rect
				{
					direction: "horizontal",
					main_align: "center",
					margin: "5 0 0",
					spacing: "10",
					width: "fill",
					
					if !bytes.is_empty()
					{
						image
						{
							image_data: dynamic_bytes(bytes),
							width: "32",
						}
					}
					
					label
					{
						font_size: "24",
						main_align: "center",
						"{game.name}"
					}
					
					Button
					{
						theme: theme_with!(ButtonTheme {
							margin: cow_borrowed!("5 0 0 0"),
						}),
						onpress: move |_| {
							if appId > 0
							{
								refresh(appId);
							}
						},
						label { "Refresh" }
					}
				}
				
				Input
				{
					onchange: move |value: String| search.set(value),
					placeholder: "Search by achievement name",
					value: search(),
					width: "50%",
				}
			}
			
			if game.hasAchievements
			{
				VirtualScrollView
				{
					cache_elements: true,
					direction: "vertical",
					item_size: 105.0,
					length: achievementsList.len(),
					scroll_controller: scrollController,
					scroll_with_arrows: true,
					
					builder: move |i, _: &Option<()>| {
						let chievo = &achievementsList[i];
						return rsx!(AchievementElement { appId, id: chievo.id.to_owned() });
					}
				}
			}
			else
			{
				label { main_align: "center", text_align: "center", width: "fill", "No Achievements to display" }
			}
		}
	);
}

fn refresh(appId: usize)
{
	spawn(async move {
		let api = SteamApi::from(SteamAuthData());
		loadGameData(&api, appId).await;
		println!("Game data loaded for {}", appId);
	});
}

async fn loadGameData(api: &SteamApi, appId: usize)
{
	if let Ok(payload) = api.getSchemaForGame(appId, &Language()).await
	{
		if let Some(game) = SteamUserData.write().games.iter_mut()
			.find(|g| g.id == appId)
		{
			game.updateAchievementsMetadata(&payload);
			println!("Done fetching game achievements for {}", appId);
			NotificationList.write().push_back("Achievements data downloaded".into());
		}
		
		if let Some(achievements) = payload.game.availableGameStats.achievements
		{
			_ = api.cacheAchievementsIcons(appId, &achievements, false).await;
			println!("Done caching achievement icons for {}!", appId);
			NotificationList.write().push_back("Achievement icons cached".into());
		}
	}
	
	if let Ok(payload) = api.getPlayerAchievements(appId, &Language()).await
	{
		if let Some(game) = SteamUserData.write().games.iter_mut()
			.find(|g| g.id == appId)
		{
			game.updateAchievementsState(&payload);
			println!("Done fetching player achievements for {}", appId);
		}
	}
	
	if let Ok(payload) = api.getGlobalPercentages(appId).await
	{
		if let Some(game) = SteamUserData.write().games.iter_mut()
			.find(|g| g.id == appId)
		{
			game.updateGlobalPercentages(&payload);
			println!("Done fetching global percentages for {}", appId);
		}
	}
	
	if let Some(game) = SteamUserData.write().games.iter_mut()
		.find(|g| g.id == appId)
	{
		game.loaded = true;
	}
	
	match saveUserData_Steam(&SteamUserData())
	{
		Err(e) => println!("Error saving user data (Steam): {:?}", e),
		Ok(_) => println!("Saved user data (Steam)"),
	}
}

fn loadIcon<'a>(game: &SteamGame) -> Vec<u8>
{
	return match loadImageToBytes(
			&SteamApi::Platform.to_lowercase(),
			&join!(Path_Games, game.id),
			&jpg!(Filename_GameIcon)
		)
	{
		Ok(bytes) => bytes,
		Err(e) => {
			println!("Error loading game icon (Steam): {:?}", e);
			vec![]
		},
	};
}
