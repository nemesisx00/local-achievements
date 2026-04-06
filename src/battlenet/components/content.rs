use freya::prelude::{Alignment, Button, ButtonLayoutThemePartialExt,
	ChildrenExt, Component, ContainerSizeExt, ContainerWithContentExt, Content,
	Direction, Element, IntoElement, Size, TextAlign, TextStyleExt, label, rect};
use freya::radio::{use_init_radio_station, use_radio};
use crate::battlenet::BattleNetGames;
use crate::data::radio::GameIdChannel;
use super::diablo3::d3Element;
use super::starcraft2::game::sc2Element;
use super::starcraft2::summary::sc2Summary;
use super::wow::wowElement;
use super::wowc::wowcElement;

#[derive(Clone, PartialEq)]
pub struct BattleNetContentElement {}

impl Component for BattleNetContentElement
{
	fn render(&self) -> impl IntoElement
	{
		use_init_radio_station::<Option<BattleNetGames>, GameIdChannel>(Default::default);
		
		let mut selectedGame = use_radio::<Option<BattleNetGames>, GameIdChannel>(GameIdChannel::BattleNet);
		
		let diabloElement: Option<Element> = match selectedGame.read()
			.is_some_and(|g| g == BattleNetGames::Diablo3)
		{
			false => None,
			true => Some(d3Element().into()),
		};
		
		let starcraftElement: Option<Element> = match selectedGame.read()
			.is_some_and(|g| g == BattleNetGames::StarCraft2)
		{
			false => None,
			true => Some(sc2Element().into()),
		};
		
		let worldOfWarcraftElement: Option<Element> = match selectedGame.read()
			.is_some_and(|g| g == BattleNetGames::WorldOfWarcraft)
		{
			false => None,
			true => Some(wowElement().into()),
		};
		
		let worldOfWarcraftClassicElement: Option<Element> = match selectedGame.read()
			.is_some_and(|g| g == BattleNetGames::WorldOfWarcraftClassic)
		{
			false => None,
			true => Some(wowcElement().into()),
		};
		
		let title = match selectedGame.read().clone()
		{
			None => "Battle.Net",
			Some(game) => game.into()
		};
		
		let displayGameSelection = diabloElement.is_none()
			&& starcraftElement.is_none()
			&& worldOfWarcraftElement.is_none()
			&& worldOfWarcraftClassicElement.is_none();
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.expanded()
			
			.child(
				rect()
					.cross_align(Alignment::Center)
					.direction(Direction::Vertical)
					.spacing(10.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.font_size(24.0)
							.text_align(TextAlign::Center)
							.width(Size::percent(100.0))
							.text(title)
					)
					
					.maybe_child(displayGameSelection.then(||
						rect()
							.direction(Direction::Vertical)
							.height(Size::percent(75.0))
							.main_align(Alignment::Center)
							.spacing(15.0)
							.width(Size::percent(100.0))
							
							.child(
								rect()
									.content(Content::Flex)
									.direction(Direction::Horizontal)
									.main_align(Alignment::Center)
									.spacing(15.0)
									.width(Size::percent(100.0))
									
									.child(
										Button::new()
											.child("Diablo III")
											.width(Size::flex(0.25))
											.on_press(move |_| **selectedGame.write() = Some(BattleNetGames::Diablo3))
									)
									
									.child(
										Button::new()
											.child("StarCraft II")
											.width(Size::flex(0.25))
											.on_press(move |_| **selectedGame.write() = Some(BattleNetGames::StarCraft2))
									)
							)
							
							.child(
								rect()
									.content(Content::Flex)
									.direction(Direction::Horizontal)
									.main_align(Alignment::Center)
									.spacing(15.0)
									.width(Size::percent(100.0))
									
									.child(
										label()
											.text("Not implemented")
											.text_align(TextAlign::Center)
											.width(Size::flex(0.25))
									)
									
									.child(
										sc2Summary()
											.width(Size::flex(0.25))
									)
							)
							
							.child(
								rect()
									.content(Content::Flex)
									.direction(Direction::Horizontal)
									.main_align(Alignment::Center)
									.spacing(15.0)
									.width(Size::percent(100.0))
									
									.child(
										Button::new()
											.child("World of Warcraft")
											.width(Size::flex(0.25))
											.on_press(move |_| **selectedGame.write() = Some(BattleNetGames::WorldOfWarcraft))
									)
									
									.child(
										Button::new()
											.child("World of Warcraft (Classic)")
											.width(Size::flex(0.25))
											.on_press(move |_| **selectedGame.write() = Some(BattleNetGames::WorldOfWarcraftClassic))
									)
							)
							
							.child(
								rect()
									.content(Content::Flex)
									.direction(Direction::Horizontal)
									.main_align(Alignment::Center)
									.spacing(15.0)
									.width(Size::percent(100.0))
									
									.child(
										label()
											.text("Not implemented")
											.text_align(TextAlign::Center)
											.width(Size::flex(0.25))
									)
									
									.child(
										label()
											.text("Not implemented")
											.text_align(TextAlign::Center)
											.width(Size::flex(0.25))
									)
							)
					))
					
					.maybe_child(diabloElement)
					.maybe_child(starcraftElement)
					.maybe_child(worldOfWarcraftElement)
					.maybe_child(worldOfWarcraftClassicElement)
			);
	}
}

impl BattleNetContentElement
{
	pub fn new() -> Self
	{
		return Self {};
	}
}
