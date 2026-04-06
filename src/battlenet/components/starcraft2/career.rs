use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, Direction,
	FontWeight, Gaps, IntoElement, Size, StyleExt, TextAlign, TextStyleExt,
	label, rect};
use freya::radio::use_radio;
use crate::battlenet::data::starcraft2::enums::SeasonLeagueType;
use crate::constants::{BorderColor, CornerRadius};
use crate::data::AppData;
use crate::data::radio::AppDataChannel;

pub fn sc2Career() -> Sc2Career
{
	return Sc2Career {};
}

#[derive(Clone, PartialEq)]
pub struct Sc2Career;

impl Component for Sc2Career
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
		
		let summary = appData.read().user.battleNet.starcraft2
			.clone()
			.unwrap_or_default()
			.career;
		
		let current1v1 = match summary.current1v1LeagueName
		{
			None => "-".into(),
			Some(name) => name.clone(),
		};
		
		let currentTeam = match summary.currentBestTeamLeagueName
		{
			None => "-".into(),
			Some(name) => name.clone(),
		};
		
		return rect()
			.border(Some(
				Border::new()
					.alignment(BorderAlignment::Center)
					.fill(BorderColor)
					.width(1.0)
			))
			.corner_radius(CornerRadius)
			.direction(Direction::Vertical)
			.padding(Gaps::new_symmetric(5.0, 10.0))
			.width(Size::percent(100.0))
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					.child(rowLabel(SeasonLeagueType::One.as_ref(), true, None))
					.child(rowLabel("Team", true, None))
			)
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					//best finish element 1v1
					//best finish element team
			)
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					.child(rowLabel(current1v1, false, None))
					.child(rowLabel(currentTeam, false, None))
			)
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					.child(rowLabel("Season Games", true, None))
					.child(rowLabel("Career Games", true, None))
			)
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					.child(rowLabel(summary.totalSeasonGames.to_string(), false, None))
					.child(rowLabel(summary.totalCareerGames.to_string(), false, None))
			)
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					.child(rowLabel("Protoss", true, Some(Size::percent(32.0))))
					.child(rowLabel("Terran", true, Some(Size::percent(32.0))))
					.child(rowLabel("Zerg", true, Some(Size::percent(32.0))))
			)
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					.child(rowLabel(summary.winsProtoss.to_string(), false, Some(Size::percent(32.0))))
					.child(rowLabel(summary.winsTerran.to_string(), false, Some(Size::percent(32.0))))
					.child(rowLabel(summary.winsZerg.to_string(), false, Some(Size::percent(32.0))))
			)
			
			;
	}
}

fn rowLabel(
	text: impl Into<String>,
	bold: impl Into<bool>,
	width: Option<Size>
) -> impl IntoElement
{
	let mut label = label()
		.text(text.into())
		.text_align(TextAlign::Center)
		.width(Size::percent(50.0));
	
	if bold.into()
	{
		label = label.font_weight(FontWeight::BOLD);
	}
	
	if width.is_some()
	{
		label = label.width(width.unwrap());
	}
	
	return label;
}
