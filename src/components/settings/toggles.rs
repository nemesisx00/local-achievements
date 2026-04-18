use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, FontWeight,
	Gaps, IntoElement, Size, Switch, TextAlign, TextStyleExt, label, rect};
use freya::radio::Writable;

#[derive(Clone, PartialEq)]
pub struct PlatformToggles
{
	battleNet: Writable<bool>,
	epicGamesStore: Writable<bool>,
	gog: Writable<bool>,
	retroAchievements: Writable<bool>,
	rpcs3: Writable<bool>,
	steam: Writable<bool>,
}

impl Component for PlatformToggles
{
	fn render(&self) -> impl IntoElement
	{
		let mut enabledBNet = self.battleNet.clone();
		let mut enabledEgs = self.epicGamesStore.clone();
		let mut enabledGog = self.gog.clone();
		let mut enabledRA = self.retroAchievements.clone();
		let mut enabledRpcs3 = self.rpcs3.clone();
		let mut enabledSteam = self.steam.clone();
		
		let labelWidth = Size::px(200.0);
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.margin(Gaps::new_all(10.0))
			.spacing(10.0)
			.width(Size::Fill)
			
			.child(
				rect()
					.content(Content::Flex)
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.font_weight(FontWeight::BOLD)
							.text("Toggle Platforms")
							.text_align(TextAlign::Center)
							.width(Size::flex(1.0))
					)
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Start)
					.width(Size::percent(75.0))
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.width(labelWidth.clone())
									.text_align(TextAlign::End)
									.text("Battle.Net")
							)
							
							.child(
								Switch::new()
									.toggled(*enabledBNet.read())
									.on_toggle(move |_| {
										let value = *enabledBNet.read();
										*enabledBNet.write() = !value;
									})
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.width(labelWidth.clone())
									.text_align(TextAlign::End)
									.text("Epic Games Store")
							)
							
							.child(
								Switch::new()
									.toggled(*enabledEgs.read())
									.on_toggle(move |_| {
										let value = *enabledEgs.read();
										*enabledEgs.write() = !value;
									})
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.width(labelWidth.clone())
									.text_align(TextAlign::End)
									.text("GOG")
							)
							
							.child(
								Switch::new()
									.toggled(*enabledGog.read())
									.on_toggle(move |_| {
										let value = *enabledGog.read();
										*enabledGog.write() = !value;
									})
							)
					)
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Start)
					.width(Size::percent(75.0))
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.width(labelWidth.clone())
									.text_align(TextAlign::End)
									.text("Retro Achievements")
							)
							
							.child(
								Switch::new()
									.toggled(*enabledRA.read())
									.on_toggle(move |_| {
										let value = *enabledRA.read();
										*enabledRA.write() = !value;
									})
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.width(labelWidth.clone())
									.text_align(TextAlign::End)
									.text("RPCS3")
							)
							
							.child(
								Switch::new()
									.toggled(*enabledRpcs3.read())
									.on_toggle(move |_| {
										let value = *enabledRpcs3.read();
										*enabledRpcs3.write() = !value;
									})
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.width(labelWidth.clone())
									.text_align(TextAlign::End)
									.text("Steam")
							)
							
							.child(
								Switch::new()
									.toggled(*enabledSteam.read())
									.on_toggle(move |_| {
										let value = *enabledSteam.read();
										*enabledSteam.write() = !value;
									})
							)
					)
			);
	}
}

impl PlatformToggles
{
	pub fn new(
		battleNet: impl Into<Writable<bool>>,
		epicGamesStore: impl Into<Writable<bool>>,
		gog: impl Into<Writable<bool>>,
		retroAchievements: impl Into<Writable<bool>>,
		rpcs3: impl Into<Writable<bool>>,
		steam: impl Into<Writable<bool>>,
	) -> Self
	{
		return Self
		{
			battleNet: battleNet.into(),
			epicGamesStore: epicGamesStore.into(),
			gog: gog.into(),
			retroAchievements: retroAchievements.into(),
			rpcs3: rpcs3.into(),
			steam: steam.into(),
		};
	}
}
