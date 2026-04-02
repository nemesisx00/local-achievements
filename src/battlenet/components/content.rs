use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt, ContainerWithContentExt, Direction, IntoElement, Size, rect};
use freya::radio::use_radio;
use crate::battlenet::components::starcraft2::Starcraft2Element;
use crate::data::AppData;
use crate::data::radio::AppDataChannel;

#[derive(Clone, PartialEq)]
pub struct BattleNetContentElement {}

impl Component for BattleNetContentElement
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.spacing(10.0)
			.width(Size::Fill)
			
			.child(format!(
				"BattleTag: {}",
				appData.read().user.battleNet.battleTag
			))
			
			.child(Starcraft2Element());
	}
}

impl BattleNetContentElement
{
	pub fn new() -> Self
	{
		return Self {};
	}
}
