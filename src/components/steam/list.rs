use freya::events::Code;
use freya::hooks::{cow_borrowed, theme_with};
use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, use_platform, use_scroll_controller, use_signal, CursorIcon, Element,
	GlobalSignal, Input, IntoDynNode, ProgressBar, ProgressBarThemeWith, Props,
	Readable, ScrollConfig, ScrollDirection, ScrollPosition, VirtualScrollView,
	Writable};
use crate::constants::{BorderColor, ButtonBackgroundColor,
	RetroAchievementsProgressColorBackground, SteamContrast, TransparentColor};
use crate::data::SteamGame;
use crate::io::{loadImageToBytes, Filename_GameIcon, Path_Games};
use crate::platforms::steam::SteamApi;
use crate::{join, jpg, SelectedGameId, SteamUserData};

#[component]
pub fn GameList() -> Element
{
	let mut scrollController = use_scroll_controller(|| ScrollConfig::default());
	let mut search = use_signal(|| String::default());
	
	let mut games = SteamUserData().games.iter()
		.filter(|g| g.name.to_lowercase().contains(&search().to_lowercase()))
		.cloned()
		.collect::<Vec<_>>();
	games.sort();
	
	return rsx!(
		rect
		{
			direction: "vertical",
			cross_align: "center",
			spacing: "10",
			width: "fill",
			
			onglobalkeyup: move |e| match e.code
			{
				Code::Home => scrollController.scroll_to(ScrollPosition::Start, ScrollDirection::Vertical),
				Code::End => scrollController.scroll_to(ScrollPosition::End, ScrollDirection::Vertical),
				_ => {},
			},
			
			Input
			{
				placeholder: "Search by game title",
				value: search(),
				width: "50%",
				onchange: move |value| search.set(value),
			}
			
			VirtualScrollView
			{
				cache_elements: true,
				direction: "vertical",
				item_size: 105.0,
				length: games.len(),
				scroll_controller: scrollController,
				scroll_with_arrows: true,
				
				builder: move |i, _: &Option<()>| {
					let game = &games[i];
					return rsx!(GameListNode { game: game.to_owned() });
				}
			}
		}
	);
}

#[component]
pub fn GameListNode(game: SteamGame) -> Element
{
    let platform = use_platform();
	
	let mut hovering = use_signal(|| false);
	let bytes = loadIcon(&game);
	
	let background = match hovering()
	{
		false => TransparentColor,
		true => ButtonBackgroundColor,
	};
	
	let percentUnlocked = game.percentUnlocked();
	let percentUnlockedString = format!("{:.2}", percentUnlocked);
	
	return rsx!(
		rect
		{
			direction: "horizontal",
			main_align: "space-around",
			margin: "5 0",
			width: "fill",
			
			rect
			{
				background,
				border: "1 center {BorderColor}",
				corner_radius: "5",
				direction: "horizontal",
				main_align: "space-between",
				min_width: "540",
				padding: "10 15",
				spacing: "10",
				width: "50%",
				
				onclick: move |_| {
					platform.set_cursor(Default::default());
					*SelectedGameId.write() = Some(game.id);
				},
				
				onpointerenter: move |_| {
					platform.set_cursor(CursorIcon::Pointer);
					hovering.set(true);
				},
				
				onpointerleave: move |_| {
					platform.set_cursor(Default::default());
					hovering.set(false);
				},
				
				rect
				{
					direction: "horizontal",
					spacing: "15",
					
					if !bytes.is_empty()
					{
						image
						{
							image_data: dynamic_bytes(bytes),
							width: "64",
						}
					}
					
					rect
					{
						direction: "vertical",
						main_align: "space-around",
						
						label
						{
							margin: "10 0 0 0",
							font_size: "18",
							"{game.name}"
						}
					}
				}
				
				rect
				{
					cross_align: "end",
					direction: "vertical",
					main_align: "space-around",
					min_width: "150",
					height: "100%",
					width: "100",
					
					if game.hasAchievements
					{
						rect
						{
							layer: "2",
							position: "absolute",
							position_right: "0",
							position_top: "10",
							width: "100",
							
							ProgressBar
							{
								progress: percentUnlocked as f32,
								theme: theme_with!(ProgressBarTheme {
									background: cow_borrowed!(RetroAchievementsProgressColorBackground),
									height: cow_borrowed!("8"),
									progress_background: cow_borrowed!(SteamContrast),
								}),
							}
						}
						
						label
						{
							margin: "10 0 0 0",
							font_size: "10",
							text_align: "center",
							width: "100",
							"{percentUnlockedString}%"
						}
					}
					else if game.loaded
					{
						label
						{
							font_size: "10",
							text_align: "center",
							width: "100",
							"Achievements N/A"
						}
					}
					else
					{
						label
						{
							font_size: "10",
							text_align: "center",
							width: "100",
							"Click to Load"
						}
					}
				}
			}
		}
	);
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
			println!("Error loading game list node icon: {:?}", e);
			vec![]
		},
	};
}
