use data::constants::{BorderColor, CornerRadius};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, Content, Direction,
	FontWeight, Gaps, IntoElement, Size, StyleExt, TextAlign, TextStyleExt,
	label, rect};
use crate::data::starcraft2::enums::SeasonLeagueType;
use super::league::sc2League;

pub fn sc2Snapshot() -> Sc2Snapshot
{
	return Sc2Snapshot {};
}

#[derive(Clone, PartialEq)]
pub struct Sc2Snapshot;

impl Component for Sc2Snapshot
{
	fn render(&self) -> impl IntoElement
	{
		return rect()
			.border(Some(Border::new()
				.alignment(BorderAlignment::Center)
				.fill(BorderColor)
				.width(1.0)
			))
			.content(Content::Flex)
			.corner_radius(CornerRadius)
			.direction(Direction::Vertical)
			.padding(Gaps::new_symmetric(5.0, 10.0))
			.spacing(5.0)
			.width(Size::percent(100.0))
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.spacing(5.0)
					.width(Size::percent(100.0))
					
					.child(
						label()
							.font_weight(FontWeight::BOLD)
							.text("League")
							.text_align(TextAlign::Center)
							.width(Size::flex(0.3))
					)
					
					.child(
						label()
							.font_weight(FontWeight::BOLD)
							.text("Rank")
							.text_align(TextAlign::Center)
							.width(Size::flex(0.3))
					)
					
					.child(
						label()
							.font_weight(FontWeight::BOLD)
							.text("Wins / Games")
							.text_align(TextAlign::Center)
							.width(Size::flex(0.3))
					)
			)
			
			.child(sc2League(SeasonLeagueType::Archon))
			.child(sc2League(SeasonLeagueType::One))
			.child(sc2League(SeasonLeagueType::Two))
			.child(sc2League(SeasonLeagueType::Three))
			.child(sc2League(SeasonLeagueType::Four));
	}
}
