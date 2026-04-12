use std::path::PathBuf;
use components::button::icon::IconButton;
use components::input::filter::AchievementsFilter;
use data::constants::Path_Games;
use data::enums::GamePlatforms;
use data::filter::{FilterCriteria, Filterable};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, Code, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Event,
	EventHandlersExt, Gaps, ImageViewer, IntoElement, KeyboardEventData,
	ScrollConfig, ScrollPosition, Size, TextAlign, TextStyleExt,
	VirtualScrollView, label, rect, use_memo, use_scroll_controller, use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::join;
use crate::data::user::Rpcs3User;
use crate::api::api::Rpcs3Api;
use super::trophy::TrophyElement;

#[derive(Clone, PartialEq)]
pub struct GameElement
{
	npCommId: String,
}

impl Component for GameElement
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<Rpcs3User, GamePlatforms>(GamePlatforms::Rpcs3);
		let mut selectedGameId = use_radio::<Option<String>, GamePlatforms>(GamePlatforms::Rpcs3);
		
		let mut scrollConroller = use_scroll_controller(ScrollConfig::default);
		
		let caseSensitive = use_state(bool::default);
		let locked = use_state(bool::default);
		let nameOnly = use_state(bool::default);
		let search = use_state(String::default);
		
		let game = user.read()
			.getGame(self.npCommId.clone())
			.unwrap_or_default();
		
		let trophies = use_memo({
			let game = game.clone();
			move || {
				game.filter(FilterCriteria
				{
					caseSensitive: caseSensitive(),
					locked: locked(),
					nameOnly: nameOnly(),
					text: search.read().clone(),
				})
			}
		});
		let trophiesLength = trophies.read().len();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: Rpcs3Api::GameIconFileName.into(),
			group: join!(Path_Games, game.npCommId),
			platform: Rpcs3Api::Platform.to_lowercase(),
		});
		
		let npCommId = self.npCommId.clone();
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.expanded()
			.margin(Gaps::new(10.0, 0.0, 5.0, 0.0))
			.spacing(10.0)
			
			.on_global_key_up(move |e: Event<KeyboardEventData>| match e.code
			{
				Code::Home => scrollConroller.scroll_to(ScrollPosition::Start, Direction::Vertical),
				Code::End => scrollConroller.scroll_to(ScrollPosition::End, Direction::Vertical),
				_ => {},
			})
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.margin(Gaps::new(5.0, 0.0, 5.0, 0.0))
					.spacing(10.0)
					.width(Size::percent(50.0))
					
					.child(
						IconButton::new(lucide::arrow_big_left())
							.alt("Back")
							.onPress(move |_| **selectedGameId.write() = None)
					)
					
					.maybe_child(filePathExists(&iconPath).then(||
						ImageViewer::new(PathBuf::from(iconPath.unwrap()))
							.height(Size::px(64.0))
					))
					
					.child(
						label()
							.font_size(24.0)
							.text_align(TextAlign::Center)
							.text(game.name)
							.width(Size::flex(0.9))
					)
			)
			
			.child(
				AchievementsFilter::new(
					caseSensitive.into_writable(),
					locked.into_writable(),
					nameOnly.into_writable(),
					search.into_writable()
				)
					.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
					.width(Size::percent(50.0))
			)
			
			.child(
				VirtualScrollView::new_controlled(
					move |i, _| {
						let trophy = &trophies.read()[i];
						return TrophyElement::new(
							npCommId.clone(),
							trophy.id
						).into();
					},
					scrollConroller
				)
					.direction(Direction::Vertical)
					.item_size(105.0)
					.length(trophiesLength)
					.scroll_with_arrows(true)
			);
	}
}

impl GameElement
{
	pub fn new(npCommId: String) -> Self
	{
		return Self
		{
			npCommId,
		};
	}
}
