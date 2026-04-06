use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt,
	ContainerWithContentExt, Content, Direction, IntoElement, Size, TextAlign,
	TextStyleExt, label, rect};
use freya::radio::use_radio;
use crate::battlenet::data::starcraft2::enums::SeasonLeagueType;
use crate::data::AppData;
use crate::data::radio::AppDataChannel;

pub fn sc2League(league: SeasonLeagueType) -> Sc2League
{
	return Sc2League
	{
		league,
	};
}

#[derive(Clone, PartialEq)]
pub struct Sc2League
{
	league: SeasonLeagueType,
}

impl Component for Sc2League
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
		
		let snapshot = appData.read().user.battleNet.starcraft2
			.clone()
			.unwrap_or_default()
			.leagueSnapshot;
		
		let league = match self.league
		{
			SeasonLeagueType::Archon => snapshot.leagues.archon,
			SeasonLeagueType::One => snapshot.leagues.one,
			SeasonLeagueType::Two => snapshot.leagues.two,
			SeasonLeagueType::Three => snapshot.leagues.three,
			SeasonLeagueType::Four => snapshot.leagues.four,
		};
		
		let name = match league.leagueName
		{
			None => self.league.as_ref().to_string(),
			Some(s) => s.clone(),
		};
		
		let rank = match league.rank < 0
		{
			false => format!("Rank {}", league.rank),
			true => "-".into(),
		};
		
		return rect()
			.content(Content::Flex)
			.direction(Direction::Horizontal)
			.main_align(Alignment::SpaceBetween)
			.spacing(5.0)
			.width(Size::percent(100.0))
			
			.child(
				label()
					.text(name)
					.text_align(TextAlign::Center)
					.width(Size::flex(0.3))
			)
			
			.child(
				label()
					.text(rank)
					.text_align(TextAlign::Center)
					.width(Size::flex(0.3))
			)
			
			.child(
				label()
					.text(format!("{} / {}", league.totalWins, league.totalGames))
					.text_align(TextAlign::Center)
					.width(Size::flex(0.3))
			);
	}
}
