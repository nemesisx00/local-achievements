use data::constants::TextColor;
use freya::icons::lucide;
use freya::prelude::{Alignment, Checkbox, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	IntoElement, Size, Tile, WritableUtils, rect, use_state};
use freya::radio::Writable;
use crate::button::icon::IconButton;

#[derive(Clone, PartialEq)]
pub struct AchievementsFilter
{
	caseSensitive: Writable<bool>,
	locked: Writable<bool>,
	margin: Gaps,
	nameOnly: Writable<bool>,
	search: Writable<String>,
	width: Size,
}

impl Component for AchievementsFilter
{
	fn render(&self) -> impl IntoElement
	{
		let mut showAdvanced = use_state(bool::default);
		
		let caseSensitive = self.caseSensitive.clone();
		let locked = self.locked.clone();
		let nameOnly = self.nameOnly.clone();
		let search = self.search.clone();
		
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
							.placeholder("Search by achievement name")
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
							.leading("Locked Only")
							.on_select({
								let mut locked = locked.clone();
								move |_| {
									let value = !*locked.read();
									locked.set(value);
								}
							})
							.child(
								Checkbox::new()
									.selected(*locked.read())
							)
					)
					
					.child(
						Tile::new()
							.leading("Case Sensitive")
							.on_select({
								let mut caseSensitive = caseSensitive.clone();
								move |_| {
									let value = !*caseSensitive.read();
									caseSensitive.set(value);
								}
							})
							.child(
								Checkbox::new()
									.selected(*caseSensitive.read())
							)
					)
					
					.child(
						Tile::new()
							.leading("Name Only")
							.on_select({
								let mut nameOnly = nameOnly.clone();
								move |_| {
									let value = !*nameOnly.read();
									nameOnly.set(value);
								}
							})
							.child(
								Checkbox::new()
									.selected(*nameOnly.read())
							)
					)
			));
	}
}

impl AchievementsFilter
{
	pub fn new(
		caseSensitive: impl Into<Writable<bool>>,
		locked: impl Into<Writable<bool>>,
		nameOnly: impl Into<Writable<bool>>,
		search: impl Into<Writable<String>>
	) -> Self
	{
		return Self
		{
			caseSensitive: caseSensitive.into(),
			locked: locked.into(),
			margin: Default::default(),
			nameOnly: nameOnly.into(),
			search: search.into(),
			width: Default::default(),
		};
	}
	
	pub fn margin(mut self, gaps: impl Into<Gaps>) -> Self
	{
		self.margin = gaps.into();
		return self;
	}
	
	pub fn width(mut self, size: impl Into<Size>) -> Self
	{
		self.width = size.into();
		return self;
	}
}
