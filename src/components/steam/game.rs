use dioxus::hooks::to_owned;
use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder, rsx, spawn, use_signal, Accordion, AccordionSummary, Button, Element, GlobalSignal, Input, IntoDynNode, Loader, Props, Readable, Signal, VirtualScrollView, Writable};
use crate::components::steam::achievement::AchievementElement;
use crate::data::{SteamAchievement, SteamGame};
use crate::io::{loadImageToBytes, saveUserData_Steam, Filename_GameIcon, Path_Games};
use crate::{join, jpg, SteamAuthData, SteamUserData};
use crate::platforms::steam::SteamApi;

#[component]
pub fn GameElement(appId: usize, language: String) -> Element
{
	let mut loaded = use_signal(|| false);
	let mut search = use_signal(|| String::default());
	
	let game = match SteamUserData().games.iter()
		.find(|g| g.id == appId)
	{
		None => SteamGame::default(),
		Some(g) => {
			loaded.set(g.hasAchievements && !g.achievements.is_empty());
			g.clone()
		},
	};
	
	let mut achievementsList: Vec<SteamAchievement> = game.achievements.iter()
		.cloned()
		.filter(|a| a.name.to_lowercase().contains(&search().to_lowercase())
			|| a.description.to_lowercase().contains(&search().to_lowercase()))
		.collect();
	achievementsList.sort();
	
	let bytes = loadIcon(&game);
	
	return rsx!(
		Accordion
		{
			summary: rsx!(AccordionSummary {
				rect
				{
					direction: "horizontal",
					main_align: "center",
					margin: "5 0 0",
					spacing: "10",
					width: "fill",
					onclick: {
						to_owned![language];
						move |_| {
							//TODO: When freya v0.4 releases, have this component request focus so the game list scrolls it to the top
							if !loaded() && appId > 0
							{
								refresh(appId, language.to_owned(), loaded);
							}
						}
					},
					
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
						line_height: "32",
						main_align: "center",
						"{game.name}"
					}
				}
			}),
			
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
					
					Button
					{
						onpress: {
							to_owned![language];
							move |_| {
								if appId > 0
								{
									refresh(appId, language.to_owned(), loaded);
								}
							}
						},
						label { "Refresh" }
					}
					
					Input
					{
						onchange: move |value: String| search.set(value),
						placeholder: "Search by achievement name",
						value: search(),
						width: "60%",
					}
				}
				
				if game.hasAchievements
				{
					VirtualScrollView
					{
						cache_elements: true,
						direction: "vertical",
						item_size: 74.0,
						length: achievementsList.len(),
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
		}
	);
}

fn refresh(appId: usize, language: String, loaded: Signal<bool>)
{
	let mut loaded = loaded.clone();
	spawn(async move {
		loadGameData(appId, &language).await;
		println!("Game data loaded for {}", appId);
		loaded.set(true);
	});
}

async fn loadGameData(appId: usize, language: &String)
{
	let api = SteamApi::withAuth(SteamAuthData());
	
	if let Ok(payload) = api.getSchemaForGame(appId, &language).await
	{
		if let Some(game) = SteamUserData.write().games.iter_mut().find(|g| g.id == appId)
		{
			game.updateAchievementsMetadata(&payload);
			println!("Done fetching game achievements for {}", appId);
		}
		
		if let Some(achievements) = payload.game.availableGameStats.achievements
		{
			_ = api.cacheAchievementsIcons(appId, &achievements, false).await;
			println!("Done caching achievement icons for {}!", appId);
		}
	}
	
	if let Ok(payload) = api.getPlayerAchievements(appId, &language).await
	{
		if let Some(game) = SteamUserData.write().games.iter_mut().find(|g| g.id == appId)
		{
			game.updateAchievementsState(&payload);
			println!("Done fetching player achievements for {}", appId);
		}
	}
	
	if let Ok(payload) = api.getGlobalPercentages(appId).await
	{
		if let Some(game) = SteamUserData.write().games.iter_mut().find(|g| g.id == appId)
		{
			game.updateGlobalPercentages(&payload);
			println!("Done fetching global percentages for {}", appId);
		}
	}
	
	_ = saveUserData_Steam(&SteamUserData());
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
			println!("Error: {:?}", e);
			vec![]
		},
	};
}
