use battlenet::components::profile::BattleNetUserProfile;
use data::constants::{BorderColor, ButtonBackgroundColor};
use data::enums::DataChannel;
use epicgamesstore::components::profile::EgsUserProfile;
use freya::prelude::{Border, BorderAlignment, BorderWidth, ChildrenExt,
	Component, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Direction, Gaps, IntoElement, Layer, LayerExt, ScrollView, Size, StyleExt,
	TextAlign, TextStyleExt, WritableUtils, label, rect, use_state};
use freya::radio::{RadioChannel, use_radio};
use gog::components::profile::GogUserProfile;
use retroachievements::components::profile::RetroAchievementsUserProfile;
use rpcs3::components::profile::Rpcs3ProfileElement;
use steam::components::profile::SteamProfile;

const AnimationDuration: u64 = 500;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ProfileState
{
	#[default]
	Hidden,
	Hiding,
	Showing,
	Shown,
}

impl RadioChannel<ProfileState> for DataChannel {}

#[derive(Clone, PartialEq)]
pub struct ProfileElement
{
	duration: u64,
	width: f32,
}

impl Component for ProfileElement
{
	fn render(&self) -> impl IntoElement
	{
		let profileState = use_radio::<ProfileState, DataChannel>(DataChannel::ProfileState);
		
		let mut visible = use_state(|| false);
		let leftShown = self.width;
		let leftHidden = 0.0;
		
		visible.set(profileState.read().clone() != ProfileState::Hidden);
		
		let x = match visible()
		{
			false => leftHidden,
			true => leftShown,
		};
		
		return rect()
			.background(ButtonBackgroundColor)
			.border(Some(
				Border::new()
					.alignment(BorderAlignment::Center)
					.fill(BorderColor)
					.width(BorderWidth
					{
						bottom: 0.0,
						top: 0.0,
						right: 1.0,
						left: 0.0,
					})
			))
			.height(Size::Fill)
			.layer(Layer::Overlay)
			.padding(Gaps::new_all(15.0))
			.width(Size::px(x))
			
			// Profiles
			.child(
				ScrollView::new()
					.show_scrollbar(visible())
					.spacing(10.0)
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.width(Size::percent(100.0))
							
							.child(profileLabelElement("Battle.Net"))
							.child(BattleNetUserProfile())
					)
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.width(Size::percent(100.0))
							
							.child(profileLabelElement("Epic Games Store"))
							.child(EgsUserProfile())
					)
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.width(Size::percent(100.0))
							
							.child(profileLabelElement("GOG"))
							.child(GogUserProfile())
					)
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.width(Size::percent(100.0))
							
							.child(profileLabelElement("RetroAchievements"))
							.child(RetroAchievementsUserProfile())
					)
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.width(Size::percent(100.0))
							
							.child(profileLabelElement("RPCS3"))
							.child(Rpcs3ProfileElement())
					)
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.width(Size::percent(100.0))
							
							.child(profileLabelElement("Steam"))
							.child(SteamProfile())
					)
			);
	}
}

impl ProfileElement
{
	pub fn new() -> Self
	{
		return Self
		{
			duration: AnimationDuration,
			width: 250.0,
		};
	}
	
	#[allow(unused)]
	pub fn duration(mut self, duration: impl Into<u64>) -> Self
	{
		self.duration = duration.into();
		return self;
	}
	
	#[allow(unused)]
	pub fn width(mut self, width: impl Into<f32>) -> Self
	{
		self.width = width.into();
		return self;
	}
}

fn profileLabelElement(text: impl Into<String>) -> impl IntoElement
{
	return label()
		.margin(Gaps::new(0.0, 0.0, 5.0, 0.0))
		.text_align(TextAlign::Center)
		.width(Size::Fill)
		.text(text.into());
}
