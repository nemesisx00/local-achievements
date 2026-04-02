use freya::prelude::{Alignment, Button, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	InputMode, InputValidator, IntoElement, Size, TextAlign, TextStyleExt,
	label, rect, spawn, use_hook, use_side_effect, use_state};
use freya::radio::use_radio;
use tracing::{info, warn};
use crate::components::{InputModeHiddenChar, NumericInput, SettingsSwitch};
use crate::data::AppData;
use crate::data::radio::AppDataChannel;
use crate::data::secure::{getBattleNetClientAuth, removeBattleNetSession,
	setBattleNetClientId, setBattleNetClientSecret};
use crate::io::saveSettings_BattleNet;

#[derive(Clone, PartialEq)]
pub struct SettingsElement
{
	labelWidth: Size,
}

impl Component for SettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let mut appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
		
		let mut clientId = use_state(Default::default);
		let mut clientSecret = use_state(Default::default);
		let inputModeClientId = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let inputModeClientSecret = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let redirectPort = use_state(|| appData.read().platform.battleNet.redirectPort);
		let redirectUri = use_state(|| "http://127.0.0.1".to_string());
		
		use_hook(|| {
			let auth = getBattleNetClientAuth().unwrap_or_default();
			clientId.set(auth.clientId().clone());
			clientSecret.set(auth.clientSecret().clone());
		});
		
		use_side_effect(move || {
			_ = setBattleNetClientId(clientId.read().clone());
			_ = setBattleNetClientSecret(clientSecret.read().clone());
			
			if appData.read().platform.battleNet.redirectPort != redirectPort()
			{
				appData.write().platform.battleNet.redirectPort = redirectPort();
				
				spawn(async move {
					match saveSettings_BattleNet(&appData.read().platform.battleNet)
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
			
			.child(
				label()
					.margin(Gaps::new(0.0, 0.0, 5.0, 0.0))
					.text_align(TextAlign::Center)
					.width(Size::Fill)
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
					
					.child(SettingsSwitch(inputModeClientId))
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
					
					.child(SettingsSwitch(inputModeClientSecret))
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
					.main_align(Alignment::Center)
					.spacing(5.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.margin(Gaps::new(5.0, 5.0, 0.0, 0.0))
							.min_width(Size::px(102.0))
							.text_align(TextAlign::End)
							.width(self.labelWidth.clone())
							.text("Redirect URI")
					)
					
					.child(
						Input::new(redirectUri)
							.on_validate(move |validator: InputValidator| validator.set_valid(false))
							.width(Size::flex(1.0))
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

impl SettingsElement
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
