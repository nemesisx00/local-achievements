use freya::prelude::{component, dioxus_elements, fc_to_builder, rsx, use_signal,
	Element, GlobalSignal, Input, Readable, VirtualScrollView, Writable};
use crate::components::retroachievements::game::GameElement;
use crate::RetroAchievementsUserData;

#[component]
pub fn GameList() -> Element
{
	let mut search = use_signal(|| String::default());
	
	let itemSize = 42.0;
	
	let mut games = RetroAchievementsUserData().games.iter()
		.cloned()
		.filter(|g| g.name.to_lowercase().contains(&search().to_lowercase())
			|| g.system.name.to_lowercase().contains(&search().to_lowercase()))
		.collect::<Vec<_>>();
	games.sort();
	
	return rsx!(
		rect
		{
			direction: "vertical",
			cross_align: "center",
			spacing: "10",
			width: "fill",
			
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
				item_size: itemSize,
				length: games.len(),
				
				builder: move |i, _: &Option<()>| {
					let game = &games[i];
					return rsx!(GameElement { gameId: game.id });
				}
			}
		}
	);
}
