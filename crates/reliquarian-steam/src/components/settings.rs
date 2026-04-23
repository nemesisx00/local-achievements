use components::extensions::PressableExt;
use components::settings::switch::InputModeSwitch;
use components::settings::util::separatorElement;
use data::constants::{BorderColor, ButtonBackgroundColor, CornerRadius,
	InputModeHiddenChar, LinkBlue};
use data::enums::GamePlatforms;
use freya::prelude::{AccessibilityExt, Alignment, Border, BorderAlignment,
	BorderWidth, ChildrenExt, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Content, Direction, Gaps, Input, InputMode,
	IntoElement, Size, StyleExt, Switch, TextAlign, TextDecoration,
	TextStyleExt, WritableUtils, label, rect, use_hook, use_side_effect,
	use_state};
use freya::radio::use_radio;
use tracing::{info, warn};
use crate::data::io::saveSettings;
use crate::data::settings::SteamSettings;
use crate::secure::{getSteamAuth, getSteamWebToken,
	removeSteamWebToken, setSteamApiKey, setSteamWebToken,
	setSteamId};

#[derive(Clone, PartialEq)]
pub struct SteamSettingsElement
{
	labelWidth: Size,
}

impl Component for SteamSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let mut settings = use_radio::<SteamSettings, GamePlatforms>(GamePlatforms::Steam);
		
		let mut apiKey = use_state(String::default);
		let mut familyToken = use_state(String::default);
		let mut id = use_state(String::default);
		let mut toggleSharedLibrary = use_state(|| settings.read().enableSteamFamilyLibrary);
		
		let inputModeApiKey = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let inputModeFamilyToken = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let inputModeId = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		
		use_hook(|| {
			if let Ok(auth) = getSteamAuth()
			{
				apiKey.set(auth.key().clone());
				id.set(auth.id().clone());
			}
			
			if let Ok(token) = getSteamWebToken()
			{
				familyToken.set(token);
			}
		});
		
		use_side_effect(move || {
			_ = setSteamApiKey(apiKey.read().clone());
			_ = setSteamId(id.read().clone());
			
			let toggle = toggleSharedLibrary();
			
			if settings.read().enableSteamFamilyLibrary != toggle
			{
				settings.write().enableSteamFamilyLibrary = toggle;
				match saveSettings(&settings.read())
				{
					Err(e) => warn!("[Steam] Error saving settings to file: {:?}", e),
					Ok(_) => info!("[Steam] Settings saved to file"),
				}
			}
			
			if toggle
			{
				_ = setSteamWebToken(familyToken.read().clone());
			}
			else
			{
				_ = removeSteamWebToken();
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
					.width(Size::Fill)
					.text("Steam Web API Authentication")
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
							.width(self.labelWidth.clone())
							.text("Steam ID")
					)
					
					.child(
						Input::new(id)
							.mode(inputModeId.read().clone())
							.placeholder("Steam ID")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(InputModeSwitch(inputModeId))
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
							.width(self.labelWidth.clone())
							.text("Steam API Key")
					)
					
					.child(
						Input::new(apiKey)
							.mode(inputModeApiKey.read().clone())
							.placeholder("Steam Web API Key")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(InputModeSwitch(inputModeApiKey))
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(10.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.min_width(Size::px(102.0))
							.text_align(TextAlign::End)
							.width(self.labelWidth.clone())
							.text("Enable Shared Library")
					)
					
					.child(
						Switch::new()
							.toggled(toggleSharedLibrary())
							.on_toggle(move |_| toggleSharedLibrary.set(!toggleSharedLibrary()))
					)
			)
			
			.maybe_child(toggleSharedLibrary().then(||
				rect()
					.background(ButtonBackgroundColor)
					.border(Some(
						Border::new()
							.alignment(BorderAlignment::Center)
							.fill(BorderColor)
							.width(BorderWidth::from(1.0))
					))
					.content(Content::Flex)
					.corner_radius(CornerRadius)
					.direction(Direction::Vertical)
					.padding(Gaps::new_all(15.0))
					.spacing(10.0)
					.width(Size::percent(60.0))
					
					.child("In order to access the data for games shared via your Steam Family library, you will first need to log in to Steam in a web browser:")
					
					.child(
						label()
							.a11y_alt("https://store.steampowered.com/login/")
							.color(LinkBlue)
							.text("https://store.steampowered.com/login/")
							.text_decoration(TextDecoration::Underline)
							.pressable(move |_| _ = webbrowser::open("https://store.steampowered.com/login/"))
					)
					
					.child("Once you are logged in, click the following link:")
					
					.child(
						label()
							.a11y_alt("https://store.steampowered.com/pointssummary/ajaxgetasyncconfig")
							.color(LinkBlue)
							.text("https://store.steampowered.com/pointssummary/ajaxgetasyncconfig")
							.text_decoration(TextDecoration::Underline)
							.pressable(move |_| _ = webbrowser::open("https://store.steampowered.com/pointssummary/ajaxgetasyncconfig"))
					)
					
					.child("Copy the value labeled `webapi_token` and paste it into the Access Token text box below")
			))
			
			.maybe_child(toggleSharedLibrary().then(||
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
							.width(self.labelWidth.clone())
							.text("Access Token")
					)
					
					.child(
						Input::new(familyToken)
							.mode(inputModeFamilyToken.read().clone())
							.placeholder("Steam Web Access Token")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(InputModeSwitch(inputModeFamilyToken))
			))
	}
}

impl SteamSettingsElement
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
