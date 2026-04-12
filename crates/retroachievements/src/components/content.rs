use data::enums::GamePlatforms;
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt,
	ContainerWithContentExt, Direction, Element, IntoElement, Size, rect};
use freya::radio::{use_init_radio_station, use_radio};
use super::game::GameElement;
use super::list::GameList;

#[derive(Clone, PartialEq)]
pub struct RetroAchievementsContent {}

impl Component for RetroAchievementsContent
{
	fn render(&self) -> impl IntoElement
	{
		use_init_radio_station::<Option<u64>, GamePlatforms>(Default::default);
		
		let selectedGameId = use_radio::<Option<u64>, GamePlatforms>(GamePlatforms::RetroAchievements);
		
		let selectedId = selectedGameId.read().clone();
		let element: Option<Element> = match selectedId
		{
			None => Some(GameList::new().into()),
			Some(id) => Some(GameElement::new(id).into()),
		};
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.spacing(10.0)
			.width(Size::Fill)
			
			.maybe_child(element);
	}
}

impl RetroAchievementsContent
{
	pub fn new() -> Self
	{
		return Self {};
	}
}
