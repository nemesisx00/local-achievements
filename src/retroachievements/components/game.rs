use freya::events::Code;
use freya::hooks::cow_borrowed;
use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, spawn, theme_with, use_hook, use_memo, use_scroll_controller,
	use_signal, Button, ButtonThemeWith, Element, GlobalSignal, Input,
	IntoDynNode, Loader, Props, Readable, ScrollConfig, ScrollDirection,
	ScrollPosition, Signal, VirtualScrollView, Writable};
use crate::io::{loadImageToBytes, saveUserData_RetroAchievements,
	FileName_GameIcon, Path_Games};
use crate::{GameSelected, NotificationList, RetroAchievementsAuthData,
	RetroAchievementsUserData, join, png};
use crate::retroachievements::data::achievement::Achievement;
use crate::retroachievements::data::game::Game;
use crate::retroachievements::platform::api::Api;
use super::SelectedGameId;
use super::achievement::AchievementElement;

#[component]
pub fn GameElement(gameId: u64) -> Element
{
	let mut loaded = use_signal(|| false);
	let mut scrollController = use_scroll_controller(|| ScrollConfig::default());
	let mut search = use_signal(|| String::default());
	
	let game = match RetroAchievementsUserData().games.iter()
		.find(|g| g.id == gameId)
	{
		None => Game::default(),
		Some(g) => {
			loaded.set(!g.achievements.is_empty());
			g.to_owned()
		},
	};
	
	let mut achievementsList: Vec<Achievement> = game.achievements.iter()
		.filter(|a| a.name.to_lowercase().contains(&search().to_lowercase())
			|| a.description.to_lowercase().contains(&search().to_lowercase()))
		.cloned()
		.collect();
	achievementsList.sort();
	
	let bytes = loadIcon(&game);
	
	use_hook(|| if !loaded() && gameId > 0
	{
		refresh(gameId, loaded);
	});
	
	use_memo(move || {
		if !GameSelected()
		{
			*SelectedGameId.write() = None;
		}
	});
	
	return rsx!(
		if !loaded()
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
						"{game.name} ({game.system.name})"
					}
					
					Button
					{
						theme: theme_with!(ButtonTheme {
							margin: cow_borrowed!("5 0 0 0"),
						}),
						onpress: move |_| {
							if gameId > 0
							{
								refresh(gameId, loaded);
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
					return rsx!(AchievementElement { gameId, achievementId: chievo.id });
				}
			}
		}
	);
}

fn refresh(gameId: u64, loaded: Signal<bool>)
{
	let mut loaded = loaded.clone();
	spawn(async move {
		let api = Api::from(RetroAchievementsAuthData());
		loadGameData(&api, gameId).await;
		println!("Game data loaded for {}", gameId);
		loaded.set(true);
	});
}

async fn loadGameData(api: &Api, gameId: u64)
{
	let ulid = match RetroAchievementsUserData().ulid
	{
		None => RetroAchievementsUserData().username,
		Some(ulid) => ulid,
	};
	
	match api.getGameInfo(&ulid, gameId).await
	{
		Err(e) => {
			println!("Error getting game info for {}: {:?}", gameId, e);
			NotificationList.write().push_back("Error downloading data".into());
		},
		
		Ok(payload) => {
			NotificationList.write().push_back("Achievements data downloaded".into());
			
			match RetroAchievementsUserData.write().games.iter_mut()
				.find(|g| g.id == gameId)
			{
				None => RetroAchievementsUserData.write().games.push(payload.to_owned().into()),
				Some(game) => game.updateDetailed(&payload),
			}
			
			match api.cacheIcon_Achievements(gameId, &payload, false).await
			{
				Err(e) => {
					println!("Error caching achievement icons for {}: {:?}", gameId, e);
					NotificationList.write().push_back("Error caching achievement icons".into());
				},
				
				Ok(_) => {
					println!("Finished caching achievement icons for {}", gameId);
					NotificationList.write().push_back("Achievement icons cached".into());
				},
			}
		}
	}
	
	match saveUserData_RetroAchievements(&RetroAchievementsUserData())
	{
		Err(e) => println!("Error saving user data (RetroAchievements): {:?}", e),
		Ok(_) => println!("Saved user data (RetroAchievements)"),
	}
}

fn loadIcon<'a>(game: &Game) -> Vec<u8>
{
	return match loadImageToBytes(
			&Api::Platform.to_lowercase(),
			&join!(Path_Games, game.id),
			&png!(FileName_GameIcon)
		)
	{
		Ok(bytes) => bytes,
		Err(e) => {
			println!("Error loading game icon (Steam): {:?}", e);
			vec![]
		},
	};
}
