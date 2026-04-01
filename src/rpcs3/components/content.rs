use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt,
	ContainerWithContentExt, Direction, Element, IntoElement, Size, rect};
use freya::radio::{use_init_radio_station, use_radio};
use crate::data::radio::GameIdChannel;
use crate::rpcs3::components::game::GameElement;
use crate::rpcs3::components::list::GameList;

#[derive(Clone, PartialEq)]
pub struct Rpcs3ContentElement {}

impl Component for Rpcs3ContentElement
{
	fn render(&self) -> impl IntoElement
	{
		use_init_radio_station::<Option<String>, GameIdChannel>(Default::default);
		
		let selectedGameId = use_radio::<Option<String>, GameIdChannel>(GameIdChannel::Rpcs3);
		
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

impl Rpcs3ContentElement
{
	pub fn new() -> Self
	{
		return Self {};
	}
}
