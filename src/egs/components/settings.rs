use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	InputMode, IntoElement, Size, TextAlign, TextStyleExt, label, rect,
	use_memo, use_state};
use crate::components::{InputModeHiddenChar, SettingsSwitch};
use crate::data::secure::{getEpicGamesStoreAccountId,
	setEpicGamesStoreAccountId};

#[derive(Clone, PartialEq)]
pub struct EgsSettingsElement
{
	labelWidth: Size,
}

impl Component for EgsSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let accountId = use_state(|| getEpicGamesStoreAccountId().unwrap_or_default());
		let inputModeId = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		
		use_memo(move || {
			_ = setEpicGamesStoreAccountId(accountId.read().clone());
		});
		
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
					.text("Epic Games Stores")
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(5.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.margin(Gaps::new(5.0, 5.0, 0.0, 0.0))
							.min_width(Size::px(102.0))
							.text_align(TextAlign::End)
							.width(self.labelWidth.clone())
							.text("Epic Account ID")
					)
					
					.child(
						Input::new(accountId)
							.mode(inputModeId.read().clone())
							.placeholder("Epic Account ID")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(SettingsSwitch(inputModeId))
			);
	}
}

impl EgsSettingsElement
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
