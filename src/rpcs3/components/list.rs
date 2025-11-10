use freya::events::Code;
use freya::prelude::{CursorIcon, Element, GlobalSignal, Input, IntoDynNode,
	ProgressBar, ProgressBarThemeWith, Props, Readable, ScrollConfig,
	ScrollDirection, ScrollPosition, VirtualScrollView, Writable, component,
	cow_borrowed, dioxus_elements, dynamic_bytes, fc_to_builder, rsx,
	theme_with, use_platform, use_scroll_controller, use_signal};
use crate::constants::{BorderColor, ButtonBackgroundColor,
	RetroAchievementsProgressColorBackground, RetroAchievementsProgressColorCasual,
	TransparentColor};
use crate::io::{loadImageToBytes, Path_Games};
use crate::{GameSelected, Rpcs3UserData, join};
use crate::rpcs3::data::game::Game;
use crate::rpcs3::platform::api::Api;
use super::SelectedGameId;

#[component]
pub fn GameList() -> Element
{
	let mut scrollController = use_scroll_controller(|| ScrollConfig::default());
	let mut search = use_signal(|| String::default());
	
	let mut games = Rpcs3UserData().games.iter()
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
pub fn GameListNode(game: Game) -> Element
{
    let platform = use_platform();
	
	let mut hovering = use_signal(|| false);
	let bytes = loadIcon(&game);
	
	let background = match hovering()
	{
		false => TransparentColor,
		true => ButtonBackgroundColor,
	};
	
	let progress = game.percentUnlocked();
	let progressString = format!("{:.2}", progress);
	
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
					platform.set_cursor(CursorIcon::default());
					*GameSelected.write() = true;
					*SelectedGameId.write() = Some(game.npCommId.to_owned());
				},
				
				onpointerenter: move |_| {
					platform.set_cursor(CursorIcon::Pointer);
					hovering.set(true);
				},
				
				onpointerleave: move |_| {
					platform.set_cursor(CursorIcon::default());
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
							width: "80",
						}
					}
					
					rect
					{
						direction: "vertical",
						main_align: "space-around",
						
						label { margin: "10 0 0 0", font_size: "18", "{game.name}" }
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
					
					rect
					{
						position: "absolute",
						position_right: "0",
						position_top: "10",
						width: "100",
						
						ProgressBar
						{
							progress: progress as f32,
							theme: theme_with!(ProgressBarTheme {
								background: cow_borrowed!(RetroAchievementsProgressColorBackground),
								height: cow_borrowed!("8"),
								progress_background: cow_borrowed!(RetroAchievementsProgressColorCasual),
							}),
						}
					}
					
					paragraph
					{
						margin: "10 0 0 0",
						text_align: "center",
						width: "100",
						
						text { font_size: "10", "{progressString}% " }
					}
				}
			}
		}
	);
}

fn loadIcon<'a>(game: &Game) -> Vec<u8>
{
	return match loadImageToBytes(
			&Api::Platform.to_lowercase(),
			&join!(Path_Games, game.npCommId),
			&Api::GameIconFileName.into()
		)
	{
		Ok(bytes) => bytes,
		Err(e) => {
			println!("Error: {:?}", e);
			vec![]
		},
	};
}
