use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt,
	ContainerWithContentExt, Direction, Element, IntoElement, Size, rect};
use freya::radio::{use_init_radio_station, use_radio};
use crate::data::radio::GameIdChannel;
use crate::egs::components::game::GameElement;
use crate::egs::components::list::GameList;

#[derive(Clone, PartialEq)]
pub struct EgsContentElement;

impl Component for EgsContentElement
{
	fn render(&self) -> impl IntoElement
	{
		use_init_radio_station::<Option<String>, GameIdChannel>(Option::default);
		
		let selectedGameId = use_radio::<Option<String>, GameIdChannel>(GameIdChannel::EpicGamesStore);
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
