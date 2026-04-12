use data::constants::{BorderColor, CornerRadius};
use data::enums::GamePlatforms;
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, Direction,
	FontWeight, Gaps, IntoElement, Size, StyleExt, TextAlign, TextStyleExt,
	label, rect};
use freya::radio::use_radio;
use crate::data::user::BattleNetUser;

pub fn sc2Campaigns() -> Sc2Campaigns
{
	return Sc2Campaigns {};
}

#[derive(Clone, PartialEq)]
pub struct Sc2Campaigns;

impl Component for Sc2Campaigns
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<BattleNetUser, GamePlatforms>(GamePlatforms::BattleNet);
		
		let summary = user.read().starcraft2
			.clone()
			.unwrap_or_default()
			.campaigns;
		
		let wol = match summary.wingsOfLiberty
		{
			None => label().width(Size::percent(32.0)),
			Some(dl) => label()
				.text(dl.as_ref().to_string())
				.text_align(TextAlign::Center)
				.width(Size::percent(32.0))
		};
		
		let hots = match summary.heartOfTheSwarm
		{
			None => label().width(Size::percent(32.0)),
			Some(dl) => label()
				.text(dl.as_ref().to_string())
				.text_align(TextAlign::Center)
				.width(Size::percent(32.0))
		};
		
		let lotv = match summary.legacyOfTheVoid
		{
			None => label().width(Size::percent(32.0)),
			Some(dl) => label()
				.text(dl.as_ref().to_string())
				.text_align(TextAlign::Center)
				.width(Size::percent(32.0))
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
				label()
					.font_weight(FontWeight::BOLD)
					.text("Campaign Difficulty Completed")
					.text_align(TextAlign::Center)
					.width(Size::percent(100.0))
			)
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					.child(
						label()
							.font_weight(FontWeight::BOLD)
							.text("WoL")
							.text_align(TextAlign::Center)
							.width(Size::percent(32.0))
					)
					
					.child(
						label()
							.font_weight(FontWeight::BOLD)
							.text("HotS")
							.text_align(TextAlign::Center)
							.width(Size::percent(32.0))
					)
					
					.child(
						label()
							.font_weight(FontWeight::BOLD)
							.text("LotV")
							.text_align(TextAlign::Center)
							.width(Size::percent(32.0))
					)
			)
			
			.child(
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.width(Size::percent(100.0))
					
					.child(wol)
					.child(hots)
					.child(lotv)
			);
	}
}
