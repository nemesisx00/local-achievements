use components::button::icon::IconButton;
use data::enums::GamePlatforms;
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt,
	ContainerWithContentExt, Direction, IntoElement, Size, rect};
use freya::radio::use_radio;
use crate::data::games::BattleNetGames;
use crate::data::user::BattleNetUser;

pub fn d3Element() -> Diablo3Element
{
	return Diablo3Element {};
}

#[derive(Clone, PartialEq)]
pub struct Diablo3Element;

impl Component for Diablo3Element
{
	fn render(&self) -> impl IntoElement
	{
		let _user = use_radio::<BattleNetUser, GamePlatforms>(GamePlatforms::BattleNet);
		let mut selectedGame = use_radio::<BattleNetGames, GamePlatforms>(GamePlatforms::BattleNet);
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.height(Size::percent(100.0))
			.spacing(10.0)
			.width(Size::percent(100.0))
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(10.0)
					.width(Size::percent(100.0))
					
					.child(
						IconButton::new(lucide::arrow_big_left())
							.alt("Back")
							.onPress(move |_| **selectedGame.write() = BattleNetGames::None)
					)
					
			);
	}
}
