use std::borrow::Cow;
use components::settings::util::separatorElement;
use data::io::{getCacheDir, getConfigDir, getDataDir};
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	InputValidator, IntoElement, Size, TextAlign, TextStyleExt, Writable, label,
	rect, use_state};

#[derive(Clone, Default, PartialEq)]
pub struct LocalInfo
{
	labelWidth: Option<Size>,
}

impl Component for LocalInfo
{
	fn render(&self) -> impl IntoElement
	{
		let labelWidth = match self.labelWidth.clone()
		{
			None => Size::percent(20.0),
			Some(lw) => lw,
		};
		
		let cacheDir = use_state(|| match getCacheDir(false)
		{
			None => String::default(),
			Some(dir) => dir
		});
		
		let configDir = use_state(|| match getConfigDir(false)
		{
			None => String::default(),
			Some(dir) => dir
		});
		
		let dataDir = use_state(|| match getDataDir(false)
		{
			None => String::default(),
			Some(dir) => dir
		});
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.margin(Gaps::new_all(10.0))
			.spacing(5.0)
			.width(Size::Fill)
			
			.child(separatorElement())
			
			.child(
				label()
					.margin(Gaps::new(0.0, 0.0, 5.0, 0.0))
					.text_align(TextAlign::Center)
					.width(Size::Fill)
					.text("Local Directories")
			)
			
			.child(directoryDisplay("Cache", labelWidth.clone(), cacheDir))
			.child(directoryDisplay("Configuration", labelWidth.clone(), configDir))
			.child(directoryDisplay("Data", labelWidth.clone(), dataDir));
	}
}

impl LocalInfo
{
	pub fn new() -> Self
	{
		return Self::default();
	}
	
	#[allow(unused)]
	pub fn labelWidth(mut self, width: impl Into<Size>) -> Self
	{
		self.labelWidth = Some(width.into());
		return self;
	}
}

fn directoryDisplay(
	labelText: impl Into<Cow<'static, str>>,
	labelWidth: impl Into<Size>,
	value: impl Into<Writable<String>>,
) -> impl IntoElement
{
	return rect()
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
				.width(labelWidth)
				.text(labelText)
		)
		
		.child(
			Input::new(value)
				.width(Size::flex(1.0))
				.on_validate(move |validator: InputValidator| validator.set_valid(false))
		);
}
