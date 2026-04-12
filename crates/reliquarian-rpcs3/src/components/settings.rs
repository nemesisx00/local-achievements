use components::input::number::NumericInput;
use data::enums::GamePlatforms;
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	IntoElement, Size, TextAlign, TextStyleExt, label, rect, spawn,
	use_side_effect, use_state};
use freya::radio::use_radio;
use tracing::{info, warn};
use crate::data::io::saveSettings;
use crate::data::settings::Rpcs3Settings;

#[derive(Clone, PartialEq)]
pub struct Rpcs3SettingsElement
{
	labelWidth: Size,
}

impl Component for Rpcs3SettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let mut settings = use_radio::<Rpcs3Settings, GamePlatforms>(GamePlatforms::Rpcs3);
		
		let accountId = use_state(|| settings.read().accountId);
		let appDataDir = use_state(|| settings.read().appDataDirectory.clone());
		
		use_side_effect(move || {
			let dir = appDataDir.read().clone();
			
			if settings.read().accountId != accountId()
				|| settings.read().appDataDirectory != dir
			{
				settings.write().accountId = accountId();
				settings.write().appDataDirectory = dir;
				
				spawn(async move {
					match saveSettings(&settings.read())
					{
						Err(e) => warn!("[RPCS3] Error saving settings: {:?}", e),
						Ok(_) => info!("[RPCS3] Saved settings"),
					}
				});
			}
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
					.text("RPCS3")
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
							.text("Account ID")
					)
					
					.child(
						NumericInput::new(accountId)
							.width(Size::flex(1.0))
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
							.text("App Data Directory")
					)
					
					.child(
						Input::new(appDataDir)
							.placeholder("RPCS3 Application Data Directory")
							.width(Size::flex(1.0))
					)
			)
		;
	}
}

impl Rpcs3SettingsElement
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
