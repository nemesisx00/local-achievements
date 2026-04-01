use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	IntoElement, MenuItem, Select, Size, TextAlign, TextStyleExt, label, rect,
	use_side_effect, use_state};
use freya::radio::use_radio;
use strum::IntoEnumIterator;
use crate::data::radio::AppDataChannel;
use crate::data::{ActiveContent, AppData};

#[derive(Clone, PartialEq)]
pub struct UiSettings
{
	labelWidth: Size,
}

impl Component for UiSettings
{
	fn render(&self) -> impl IntoElement
	{
		let mut appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Settings);
		
		let mut defaultActiveContent = use_state(|| appData.read().app.settings.defaultActivePlatform);
		let language = use_state(|| appData.read().app.settings.language.clone());
		
		use_side_effect(move || {
			appData.write().app.settings.defaultActivePlatform = defaultActiveContent.read().clone();
			appData.write().app.settings.language = language.read().clone();
		});
		
		let labelWidth = self.labelWidth.clone();
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.margin(Gaps::new_all(10.0))
			.spacing(5.0)
			.width(Size::Fill)
			
			.child(
				label()
					.margin(Gaps::new(0.0, 0.0, 5.0, 0.0))
					.text_align(TextAlign::Center)
					.width(Size::Fill)
					.text("Application")
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Start)
					.spacing(10.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.margin(Gaps::new(7.0, 0.0, 0.0, 0.0))
							.min_width(Size::px(102.0))
							.text_align(TextAlign::End)
							.width(labelWidth.clone())
							.text("Starting Tab")
					)
					
					.child(
						Select::new()
							.selected_item(
								match ActiveContent::iter()
									.find(|ac| ac == &defaultActiveContent())
								{
									None => ActiveContent::default().to_string(),
									Some(ac) => ac.to_string(),
								}
							)
							.children(
								ActiveContent::iter().map(|ac| {
									MenuItem::new()
										.selected(ac == defaultActiveContent())
										.on_press(move |_| defaultActiveContent.set(ac))
										.child(ac.to_string())
										.into()
								})
							)
					)
					
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(10.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.margin(Gaps::new(7.0, 0.0, 0.0, 0.0))
							.min_width(Size::px(102.0))
							.text_align(TextAlign::End)
							.width(labelWidth.clone())
							.text("Language")
					)
					
					.child(
						Input::new(language)
							.placeholder("en")
							.width(Size::flex(1.0))
					)
			);
	}
}

impl UiSettings
{
	pub fn new() -> Self
	{
		return Self
		{
			labelWidth: Size::percent(20.0),
		};
	}
	
	#[allow(unused)]
	pub fn labelWidth(mut self, width: impl Into<Size>) -> Self
	{
		self.labelWidth = width.into();
		return self;
	}
}
