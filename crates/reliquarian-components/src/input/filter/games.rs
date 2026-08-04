use data::constants::TextColor;
use data::enums::DataChannel;
use data::io::saveAppSettings;
use data::settings::AppSettings;
use freya::elements::extensions::ContainerPositionExt;
use freya::icons::lucide;
use freya::prelude::{Alignment, Checkbox, ChildrenExt, Component,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	IntoElement, Size, Tile, WritableUtils, rect, spawn, use_side_effect,
	use_state};
use freya::radio::{Writable, use_radio};
use tracing::{info, warn};
use crate::button::icon::IconButton;

#[derive(Clone, PartialEq)]
pub struct GamesFilter
{
	margin: Gaps,
	nameOnly: Option<bool>,
	search: Writable<String>,
	showAll: Option<bool>,
	width: Size,
}

impl Component for GamesFilter
{
	fn render(&self) -> impl IntoElement
	{
		let mut appSettings = use_radio::<AppSettings, DataChannel>(DataChannel::Settings);
		
		let mut filterCriteria = use_state(|| appSettings.read().filterCriteria);
		let mut showAdvanced = use_state(bool::default);
		
		let caseSensitive = filterCriteria().caseSensitive;
		let nameOnly = filterCriteria().nameOnly;
		let search = self.search.clone();
		let showAll = filterCriteria().showAll;
		
		use_side_effect(move || {
			appSettings.write().filterCriteria = filterCriteria();
			
			spawn(async move {
				match saveAppSettings(&appSettings.read())
				{
					Err(e) => warn!("[Reliquarian] Error saving app settings: {:?}", e),
					Ok(_) => info!("[Reliquarian] Saved app settings"),
				}
			});
		});
		
		return rect()
			.direction(Direction::Vertical)
			.margin(self.margin.clone())
			.width(self.width.clone())
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(5.0)
					.width(Size::percent(100.0))
					
					.child(
						Input::new(search)
							.placeholder("Search by game name")
							.width(Size::flex(1.0))
					)
					
					.child(
						IconButton::new(lucide::funnel())
							.alt("Advanced Filters")
							.color(TextColor)
							.height(Size::px(35.0))
							.innerHeight(Size::px(20.0))
							.innerWidth(Size::px(20.0))
							.width(Size::px(35.0))
							.onPress(move |_| showAdvanced.set(!showAdvanced()))
					)
			)
			
			.maybe_child(showAdvanced().then(||
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceEvenly)
					.width(Size::percent(100.0))
					
					.child(
						Tile::new()
							.leading("Case Sensitive")
							.on_select(move |_| filterCriteria.write().caseSensitive = !caseSensitive)
							.child(
								Checkbox::new()
									.selected(caseSensitive)
							)
					)
					
					.maybe_child(self.nameOnly.is_some().then(||
						Tile::new()
							.leading("Name Only")
							.on_select(move |_| filterCriteria.write().nameOnly = !nameOnly)
							.child(
								Checkbox::new()
									.selected(nameOnly)
							)
					))
					
					.maybe_child(self.showAll.is_some().then(||
						Tile::new()
							.leading("Include Games Without Achievements")
							.on_select(move |_| filterCriteria.write().showAll = !showAll)
							.child(
								Checkbox::new()
									.selected(showAll)
							)
					))
			));
	}
}

impl GamesFilter
{
	pub fn new(search: impl Into<Writable<String>>) -> Self
	{
		return Self
		{
			margin: Default::default(),
			nameOnly: None,
			search: search.into(),
			showAll: None,
			width: Default::default(),
		};
	}
	
	pub fn margin(mut self, gaps: impl Into<Gaps>) -> Self
	{
		self.margin = gaps.into();
		return self;
	}
	
	pub fn nameOnly(mut self, state: impl Into<bool>) -> Self
	{
		self.nameOnly = Some(state.into());
		return self;
	}
	
	pub fn showAll(mut self, state: impl Into<bool>) -> Self
	{
		self.showAll = Some(state.into());
		return self;
	}
	
	pub fn width(mut self, size: impl Into<Size>) -> Self
	{
		self.width = size.into();
		return self;
	}
}
