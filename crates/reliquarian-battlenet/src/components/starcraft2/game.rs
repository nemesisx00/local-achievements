use components::input::filter::AchievementsFilter;
use data::enums::GamePlatforms;
use data::filter::{FilterCriteria, Filterable};
use freya::prelude::{Alignment, ChildrenExt, Code, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Event,
	EventHandlersExt, Gaps, IntoElement, KeyboardEventData, ScrollConfig,
	ScrollPosition, Size, VirtualScrollView, rect, use_scroll_controller,
	use_state};
use freya::radio::{IntoWritable, use_radio};
use crate::data::user::BattleNetUser;
use super::achievement::sc2Achievement;
use super::campaigns::sc2Campaigns;
use super::career::sc2Career;
use super::level::sc2Level;
use super::snapshot::sc2Snapshot;
use super::summary::sc2Summary;

pub fn sc2Element() -> Sc2Element
{
	return Sc2Element {};
}

#[derive(Clone, PartialEq)]
pub struct Sc2Element;

impl Component for Sc2Element
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<BattleNetUser, GamePlatforms>(GamePlatforms::BattleNet);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		
		let caseSensitive = use_state(bool::default);
		let locked = use_state(bool::default);
		let nameOnly = use_state(bool::default);
		let search = use_state(String::default);
		
		let profile = user.read().starcraft2
			.clone()
			.unwrap_or_default();
		
		let achievements = profile.filter(FilterCriteria
		{
			caseSensitive: caseSensitive(),
			locked: locked(),
			nameOnly: nameOnly(),
			text: search.read().clone(),
			..Default::default()
		});
		
		let achievementsLength = achievements.len();
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.height(Size::percent(100.0))
			.spacing(10.0)
			.width(Size::percent(100.0))
			
			.on_global_key_up(move |e: Event<KeyboardEventData>| match e.code
			{
				Code::Home => scrollController.scroll_to(ScrollPosition::Start, Direction::Vertical),
				Code::End => scrollController.scroll_to(ScrollPosition::End, Direction::Vertical),
				_ => {},
			})
			
			.child(
				sc2Summary()
					.showBackButton(true)
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.height(Size::percent(100.0))
					.spacing(5.0)
					.width(Size::percent(100.0))
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Vertical)
							.height(Size::percent(100.0))
							.margin(Gaps::new_symmetric(5.0, 10.0))
							.spacing(10.0)
							.width(Size::flex(0.5))
							
							.child(sc2Campaigns())
							
							.child(
								rect()
									.direction(Direction::Horizontal)
									.main_align(Alignment::SpaceBetween)
									.width(Size::percent(100.0))
									
									.child(
										sc2Level(profile.levelProtoss)
											.label("Protoss")
											.width(Size::percent(32.0))
									)
									
									.child(
										sc2Level(profile.levelTerran)
											.label("Terran")
											.width(Size::percent(32.0))
									)
									
									.child(
										sc2Level(profile.levelZerg)
											.label("Zerg")
											.width(Size::percent(32.0))
									)
							)
							
							.child(sc2Career())
							.child(sc2Snapshot())
					)
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.height(Size::percent(100.0))
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.spacing(5.0)
							.width(Size::flex(0.5))
							
							.child(
								AchievementsFilter::new(
									caseSensitive.into_writable(),
									locked.into_writable(),
									nameOnly.into_writable(),
									search.into_writable()
								)
									.width(Size::percent(100.0))
							)
							
							.child(
								VirtualScrollView::new_controlled(
									move |i, _| {
										let id = achievements[i].id;
										sc2Achievement(id).into()
									},
									scrollController
								)
									.direction(Direction::Vertical)
									.height(Size::percent(100.0))
									.item_size(81.0)
									.length(achievementsLength)
									.scroll_with_arrows(true)
									.width(Size::percent(100.0))
							)
					)
			);
	}
}
