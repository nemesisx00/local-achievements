use freya::events::Code;
use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, use_memo, use_scroll_controller, use_signal, Element, GlobalSignal,
	Input, IntoDynNode, Props, Readable, ScrollConfig, ScrollDirection,
	ScrollPosition, VirtualScrollView, Writable};
use crate::io::{loadImageToBytes, Path_Games};
use crate::rpcs3::components::SelectedGameId;
use crate::{GameSelected, Rpcs3UserData, join};
use crate::rpcs3::data::game::Game;
use crate::rpcs3::data::trophy::Trophy;
use crate::rpcs3::platform::api::Api;
use super::trophy::TrophyElement;

#[component]
pub fn GameElement(npCommId: String) -> Element
{
	let mut scrollController = use_scroll_controller(|| ScrollConfig::default());
	let mut search = use_signal(|| String::default());
	
	let game = match Rpcs3UserData().games.iter()
		.find(|g| g.npCommId == npCommId)
	{
		None => Game::default(),
		Some(g) => g.to_owned(),
	};
	
	let mut trophiesList: Vec<Trophy> = game.trophies.iter()
		.filter(|t| t.name.to_lowercase().contains(&search().to_lowercase())
			|| t.detail.to_lowercase().contains(&search().to_lowercase()))
		.cloned()
		.collect();
	trophiesList.sort();
	
	let bytes = loadIcon(&game);
	
	use_memo(move || {
		if !GameSelected()
		{
			*SelectedGameId.write() = None;
		}
	});
	
	return rsx!(
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
						width: "80",
					}
				}
				
				label
				{
					font_size: "24",
					main_align: "center",
					"{game.name}"
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
			length: trophiesList.len(),
			scroll_controller: scrollController,
			scroll_with_arrows: true,
			
			builder: move |i, _: &Option<()>| {
				let trophy = &trophiesList[i];
				return rsx!(TrophyElement { npCommId: npCommId.to_owned(), trophyId: trophy.id });
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
			println!("Error loading game icon (RPCS3): {:?}", e);
			vec![]
		},
	};
}
