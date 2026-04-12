use data::enums::GamePlatforms;
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt,
	ContainerWithContentExt, Direction, Element, IntoElement, Size, rect};
use freya::radio::{use_init_radio_station, use_radio};
use super::game::GameElement;
use super::list::GameList;

#[derive(Clone, PartialEq)]
pub struct EgsContentElement;

impl Component for EgsContentElement
{
	fn render(&self) -> impl IntoElement
	{
		use_init_radio_station::<Option<String>, GamePlatforms>(Option::default);
		
		let selectedGameId = use_radio::<Option<String>, GamePlatforms>(GamePlatforms::EpicGamesStore);
		let selectedElement: Option<Element> = match selectedGameId.read().clone()
		{
			None => Some(GameList::new().into()),
			Some(id) => Some(GameElement::new(id).into()),
		};
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.spacing(10.0)
			.width(Size::Fill)
			
			.maybe_child(selectedElement);
	}
}

impl EgsContentElement
{
	pub fn new() -> Self
	{
		return Self {};
	}
}
