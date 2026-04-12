use battlenet::components::settings::BattleNetSettingsElement;
use data::constants::BorderColor;
use epicgamesstore::components::settings::EgsSettingsElement;
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, Component, ContainerSizeExt, ContainerWithContentExt,
	Direction, IntoElement, ScrollView, Size, StyleExt, rect};
use gog::components::settings::GogSettingsElement;
use retroachievements::components::settings::RetroAchievementsSettingsElement;
use rpcs3::components::settings::Rpcs3SettingsElement;
use steam::components::settings::SteamSettingsElement;
use crate::components::settings::local::LocalInfo;
use crate::components::settings::notifications::NotificationSettings;
use crate::components::settings::ui::UiSettings;

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
					.child(EgsSettingsElement::new())
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
