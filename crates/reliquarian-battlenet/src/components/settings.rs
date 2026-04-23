use components::input::number::NumericInput;
use components::settings::switch::InputModeSwitch;
use components::settings::util::separatorElement;
use data::constants::InputModeHiddenChar;
use data::enums::GamePlatforms;
use freya::prelude::{Alignment, Button, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	InputMode, IntoElement, MenuItem, Select, Size, TextAlign, TextStyleExt,
	WritableUtils, label, rect, spawn, use_hook, use_side_effect, use_state};
use freya::radio::use_radio;
use strum::IntoEnumIterator;
use tracing::{info, warn};
use crate::api::BattleNetSettings;
use crate::data::io::saveSettings;
use crate::data::region::Region;
use crate::secure::{getBattleNetClientAuth, removeBattleNetSession,
	setBattleNetClientId, setBattleNetClientSecret};

#[derive(Clone, PartialEq)]
pub struct BattleNetSettingsElement
{
	labelWidth: Size,
}

impl Component for BattleNetSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let mut settings = use_radio::<BattleNetSettings, GamePlatforms>(GamePlatforms::BattleNet);
		
		let mut clientId = use_state(String::default);
		let mut clientSecret = use_state(String::default);
		let inputModeClientId = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let inputModeClientSecret = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let mut defaultRegion = use_state(|| settings.read().defaultRegion);
		let redirectPort = use_state(|| settings.read().redirectPort);
		
		use_hook(|| {
			let auth = getBattleNetClientAuth().unwrap_or_default();
			clientId.set(auth.clientId().clone());
			clientSecret.set(auth.clientSecret().clone());
		});
		
		use_side_effect(move || {
			_ = setBattleNetClientId(clientId.read().clone());
			_ = setBattleNetClientSecret(clientSecret.read().clone());
			
			if settings.read().defaultRegion != defaultRegion()
				|| settings.read().redirectPort != redirectPort()
				
			{
				settings.write().defaultRegion = defaultRegion();
				settings.write().redirectPort = redirectPort();
				
				spawn(async move {
					match saveSettings(&settings.read())
					{
						Err(e) => warn!("[BattleNet] Error saving settings: {:?}", e),
						Ok(_) => info!("[BattleNet] Saved settings"),
					}
				});
			}
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
					.width(Size::percent(75.0))
					.text("Battle.Net API Authentication")
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
							.text("Client ID")
					)
					
					.child(
						Input::new(clientId)
							.mode(inputModeClientId.read().clone())
							.placeholder("Battle.Net Client ID")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(InputModeSwitch(inputModeClientId))
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
							.text("Client Secret")
					)
					
					.child(
						Input::new(clientSecret)
							.mode(inputModeClientSecret.read().clone())
							.placeholder("Battle.Net Client Secret")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(InputModeSwitch(inputModeClientSecret))
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
							.text("Redirect Port")
					)
					
					.child(
						NumericInput::new(redirectPort)
							.placeholder("8080")
							.width(Size::flex(1.0))
					)
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Start)
					.spacing(5.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.margin(Gaps::new(5.0, 5.0, 0.0, 0.0))
							.min_width(Size::px(102.0))
							.text_align(TextAlign::End)
							.width(self.labelWidth.clone())
							.text("Default Region")
					)
					
					.child(
						Select::new()
							.selected_item(defaultRegion().as_ref())
							.children(
								Region::iter().map(|ac| {
									MenuItem::new()
										.selected(ac == defaultRegion())
										.on_press(move |_| defaultRegion.set(ac))
										.child(ac.as_ref())
										.into()
								})
							)
					)
			)
			
			.child(
				label()
					.margin(Gaps::new_symmetric(5.0, 0.0))
					.text_align(TextAlign::Center)
					.width(Size::Fill)
					.text("Battle.Net Session Management")
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(10.0)
					.width(Size::percent(75.0))
					
					.child(
						Button::new()
							.on_press(move |_| match removeBattleNetSession()
							{
								Err(e) => warn!("[BattleNet] Error removing session data: {:?}", e),
								Ok(_) => info!("[BattleNet] Session data removed"),
							})
							.child("Clear Session")
					)
			);
	}
}

impl BattleNetSettingsElement
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
