use std::path::PathBuf;
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt,
	ContainerWithContentExt, Content, Direction, ImageViewer, IntoElement, Size,
	rect, spawn, use_state};
use freya::radio::use_radio;
use crate::battlenet::BattleNetGames;
use crate::battlenet::platform::api::BattleNetApi;
use crate::battlenet::platform::starcraft2::Starcraft2;
use crate::components::IconButton;
use crate::data::AppData;
use crate::data::radio::{AppDataChannel, DataChannel, GameIdChannel};
use crate::data::secure::getBattleNetSession;
use crate::io::{Path_Avatars, getImagePath};
use crate::jpgAlt;
use crate::net::limiter::RateLimiter;
use crate::net::limiter::request::{BattleNetOperation, FileLocation,
	RequestEvent};
use crate::util::filePathExists;

pub fn sc2Summary() -> Sc2Summary
{
	return Sc2Summary
	{
		showBackButton: Default::default(),
		width: Default::default(),
	};
}

#[derive(Clone, PartialEq)]
pub struct Sc2Summary
{
	showBackButton: bool,
	width: Size,
}

impl Component for Sc2Summary
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let mut selectedGame = use_radio::<Option<BattleNetGames>, GameIdChannel>(GameIdChannel::BattleNet);
		
		let mut sessionValid = use_state(bool::default);
		
		let profile = appData.read().user.battleNet.starcraft2
			.clone()
			.unwrap_or_default();
		
		let avatarPath = getImagePath(&FileLocation
		{
			fileName: jpgAlt!(Starcraft2::GamePrefix, profile.id),
			group: Path_Avatars.into(),
			platform: BattleNetApi::Platform.to_lowercase(),
		});
		
		sessionValid.set(
			getBattleNetSession()
				.is_ok_and(|s| !s.hasExpired())
		);
		
		return rect()
			.direction(Direction::Horizontal)
			.main_align(Alignment::Center)
			.spacing(10.0)
			.width(self.width.clone())
			
			.maybe_child(self.showBackButton.then(||
				IconButton::new(lucide::arrow_big_left())
					.alt("Back")
					.onPress(move |_| **selectedGame.write() = None)
			))
			
			.maybe_child(filePathExists(&avatarPath).then(||
				ImageViewer::new(PathBuf::from(avatarPath.unwrap()))
					.width(Size::px(64.0))
			))
			
			.child(
				rect()
					.direction(Direction::Vertical)
					.height(Size::px(64.0))
					.main_align(Alignment::Center)
					
					.child(
						rect()
							.direction(Direction::Horizontal)
							.spacing(5.0)
							.child(profile.name)
							.child(format!("({})", profile.region.as_ref()))
					)
					
					.child(
						rect()
							.content(Content::Flex)
							.direction(Direction::Horizontal)
							.spacing(10.0)
							
							.child(
								swarmLevelElement(
									"P",
									profile.levelProtoss.level
								)
							)
							
							.child(
								swarmLevelElement(
									"T",
									profile.levelTerran.level
								)
							)
							
							.child(
								swarmLevelElement(
									"Z",
									profile.levelZerg.level
								)
							)
					)
			)
			
			.maybe_child(sessionValid().then(||
				rect()
					.cross_align(Alignment::Center)
					.direction(Direction::Vertical)
					.height(Size::px(64.0))
					.main_align(Alignment::Center)
					
					.child(
						IconButton::new(lucide::refresh_ccw())
							.alt("Refresh")
							.height(Size::px(32.0))
							.width(Size::px(32.0))
							.onPress(move |_| {
								spawn(async move {
									rateLimiter.read().pushAll(vec![
										BattleNetOperation::GetSc2PlayerAccount.into(),
										BattleNetOperation::GetSc2StaticProfile.into(),
										BattleNetOperation::GetSc2PlayerProfile.into(),
										BattleNetOperation::SaveToFile.into(),
									]).await;
									
									**requestEvent.write() = RequestEvent::Added;
								});
							})
					)
			));
	}
}

impl Sc2Summary
{
	pub fn showBackButton(mut self, show: impl Into<bool>) -> Self
	{
		self.showBackButton = show.into();
		return self;
	}
	
	pub fn width(mut self, size: impl Into<Size>) -> Self
	{
		self.width = size.into();
		return self;
	}
}

fn swarmLevelElement(text: impl Into<String>, level: impl Into<u64>) -> impl IntoElement
{
	let text = text.into();
	let level = level.into().to_string();
	
	return rect()
		.direction(Direction::Horizontal)
		.child(format!("{} {}", text, level));
}
