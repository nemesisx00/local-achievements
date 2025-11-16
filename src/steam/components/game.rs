use freya::events::Code;
use freya::hooks::{cow_borrowed, theme_with};
use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, spawn, use_hook, use_memo, use_scroll_controller, use_signal, Button,
	ButtonThemeWith, Element, GlobalSignal, Input, IntoDynNode, Loader, Props,
	Readable, ScrollConfig, ScrollDirection, ScrollPosition, VirtualScrollView,
	Writable};
use crate::io::{loadImageToBytes, saveUserData_Steam, FileName_GameIcon,
	Path_Games};
use crate::{GameSelected, Language, NotificationList, SteamAuthData,
	SteamUserData, join, jpg};
use crate::steam::data::achievement::Achievement;
use crate::steam::data::game::Game;
use crate::steam::platform::api::Api;
use super::SelectedGameId;
use super::achievement::AchievementElement;

#[component]
pub fn GameElement(appId: u64) -> Element
{
	let mut scrollController = use_scroll_controller(|| ScrollConfig::default());
	let mut search = use_signal(|| String::default());
	
	let game = match SteamUserData().games.iter()
		.find(|g| g.id == appId)
	{
		None => Game::default(),
		Some(g) => g.clone(),
	};
	
	let mut achievementsList: Vec<Achievement> = game.achievements.iter()
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
	
	use_memo(move || {
		if !GameSelected()
		{
			*SelectedGameId.write() = None;
		}
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

fn refresh(appId: u64)
{
	spawn(async move {
		let api = Api::from(SteamAuthData());
		loadGameData(&api, appId).await;
		println!("Game data loaded for {}", appId);
	});
}

async fn loadGameData(api: &Api, appId: u64)
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

fn loadIcon<'a>(game: &Game) -> Vec<u8>
{
	return match loadImageToBytes(
			&Api::Platform.to_lowercase(),
			&join!(Path_Games, game.id),
			&jpg!(FileName_GameIcon)
		)
	{
		Ok(bytes) => bytes,
		Err(e) => {
			println!("Error loading game icon (Steam): {:?}", e);
			vec![]
		},
	};
}
