use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Direction, Gaps, InputMode, IntoElement,
	ScrollView, Size, State, StyleExt, Switch, rect};
use crate::battlenet::BattleNetSettingsElement;
use crate::components::settings::local::LocalInfo;
use crate::components::settings::notifications::NotificationSettings;
use crate::components::settings::ui::UiSettings;
use crate::constants::BorderColor;
use crate::gog::GogSettingsElement;
use crate::retroachievements::RetroAchievementsSettingsElement;
use crate::rpcs3::Rpcs3SettingsElement;
use crate::steam::SteamSettingsElement;

pub const InputModeHiddenChar: char = '*';

#[derive(Clone, Default, PartialEq)]
pub struct AppSettingsElement
{
	labelWidth: Option<Size>,
}

impl Component for AppSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		return rect()
			.direction(Direction::Vertical)
			.width(Size::Fill)
			
			.child(
				ScrollView::new()
					.spacing(15.0)
					.child(UiSettings::new())
					.child(separatorElement())
					.child(NotificationSettings::new())
					.child(separatorElement())
					.child(BattleNetSettingsElement::new())
					.child(separatorElement())
					.child(GogSettingsElement::new())
					.child(separatorElement())
					.child(RetroAchievementsSettingsElement::new())
					.child(separatorElement())
					.child(Rpcs3SettingsElement::new())
					.child(separatorElement())
					.child(SteamSettingsElement::new())
					.child(separatorElement())
					.child(LocalInfo::new())
			);
	}
}

impl AppSettingsElement
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

pub fn SettingsSwitch(mut inputMode: State<InputMode>) -> impl IntoElement
{
	let value = inputMode.read().clone();
	
	return rect()
		.margin(Gaps::new(4.0, 0.0, 0.0, 0.0))
		.width(Size::FillMinimum)
		.child(
			Switch::new()
				.toggled(value == InputMode::Shown)
				.on_toggle(move |_| match value
				{
					InputMode::Shown => inputMode.set(InputMode::Hidden(InputModeHiddenChar)),
					InputMode::Hidden(_) => inputMode.set(InputMode::Shown),
				})
		);
}

fn separatorElement() -> impl IntoElement
{
	return rect()
		.cross_align(Alignment::Center)
		.direction(Direction::Horizontal)
		.height(Size::px(1.0))
		.main_align(Alignment::Center)
		//.margin(Gaps::new_symmetric(25.0, 0.0))
		.width(Size::Fill)
		
		.child(
			rect()
				.border(Some(
					Border::new()
						.alignment(BorderAlignment::Center)
						.fill(BorderColor)
						.width(BorderWidth
						{
							top: 0.0,
							right: 0.0,
							bottom: 1.0,
							left: 0.0,
						})
				))
				.height(Size::px(1.0))
				.width(Size::percent(40.0))
		);
}
