use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, Content, Direction,
	Gaps, IntoElement, ProgressBar, ProgressBarThemePartialExt, Size, StyleExt,
	label, rect};
use crate::battlenet::data::starcraft2::profile::levels::FactionLevel;
use crate::constants::{BorderColor, CornerRadius,
	RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorHardcore};

pub fn sc2Level(level: FactionLevel) -> Sc2Level
{
	return Sc2Level
	{
		label: String::default(),
		level,
		width: Size::auto(),
	};
}

#[derive(Clone, PartialEq)]
pub struct Sc2Level
{
	label: String,
	level: FactionLevel,
	width: Size,
}

impl Component for Sc2Level
{
	fn render(&self) -> impl IntoElement
	{
		let percent = self.level.percentToNextLevel();
		
		return rect()
			.border(Some(
				Border::new()
					.alignment(BorderAlignment::Center)
					.fill(BorderColor)
					.width(1.0)
			))
			.content(Content::Flex)
			.corner_radius(CornerRadius)
			.direction(Direction::Horizontal)
			.main_align(Alignment::SpaceBetween)
			.padding(Gaps::new_symmetric(5.0, 10.0))
			.spacing(5.0)
			.width(self.width.clone())
			
			.child(
				label()
					.text(self.label.clone())
					.width(Size::flex(0.5))
			)
			
			.child(
				label()
					.text(self.level.level.to_string())
					.width(Size::flex(0.2))
			)
			
			.child(
				rect()
					.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
					.width(Size::flex(0.3))
					
					.child(
						ProgressBar::new(percent)
							.background(RetroAchievementsProgressColorBackground)
							.color(RetroAchievementsProgressColorHardcore)
							.progress_background(RetroAchievementsProgressColorHardcore)
					)
			)
			;
	}
}

impl Sc2Level
{
	pub fn label(mut self, text: impl Into<String>) -> Self
	{
		self.label = text.into();
		return self;
	}
	
	pub fn width(mut self, size: impl Into<Size>) -> Self
	{
		self.width = size.into();
		return self;
	}
}
