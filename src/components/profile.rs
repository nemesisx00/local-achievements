use freya::prelude::{Border, BorderAlignment, BorderWidth, ChildrenExt,
	Component, ContainerExt, ContainerSizeExt, ContainerWithContentExt, Gaps,
	IntoElement, Layer, LayerExt, ScrollView, Size, StyleExt, TextAlign,
	TextStyleExt, label, rect, use_state};
use freya::radio::use_radio;
use crate::battlenet::BattleNetUserProfile;
use crate::constants::{BorderColor, ButtonBackgroundColor};
use crate::data::radio::DataChannel;
use crate::egs::EgsUserProfile;
use crate::gog::GogUserProfile;
use crate::retroachievements::RetroAchievementsUserProfile;
use crate::rpcs3::Rpcs3ProfileElement;
use crate::steam::SteamProfile;

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
			.spacing(10.0)
			.width(Size::px(x))
			
			// Profiles
			.child(
				ScrollView::new()
					.show_scrollbar(visible())
					
					.child(profileLabelElement("Battle.Net"))
					.child(BattleNetUserProfile())
					
					.child(separatorElement())
					
					.child(profileLabelElement("Epic Games Store"))
					.child(EgsUserProfile())
					
					.child(separatorElement())
					
					.child(profileLabelElement("GOG"))
					.child(GogUserProfile())
					
					.child(separatorElement())
					
					.child(profileLabelElement("RetroAchievements"))
					.child(RetroAchievementsUserProfile())
					
					.child(separatorElement())
					
					.child(profileLabelElement("RPCS3"))
					.child(Rpcs3ProfileElement())
					
					.child(separatorElement())
					
					.child(profileLabelElement("Steam"))
					.child(SteamProfile())
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

fn separatorElement() -> impl IntoElement
{
	return rect()
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
		.margin(Gaps::new_symmetric(25.0, 0.0))
		.width(Size::Fill);
}
