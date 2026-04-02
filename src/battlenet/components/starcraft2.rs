use std::path::PathBuf;

use freya::prelude::{Alignment, ChildrenExt, ContainerSizeExt, ContainerWithContentExt, Direction, ImageViewer, IntoElement, Size, rect};
use freya::radio::use_radio;
use crate::battlenet::platform::api::BattleNetApi;
use crate::battlenet::platform::starcraft2::Starcraft2;
use crate::data::AppData;
use crate::data::radio::AppDataChannel;
use crate::io::{Path_Avatars, getImagePath};
use crate::jpgAlt;
use crate::net::limiter::request::FileLocation;

pub fn Starcraft2Element() -> impl IntoElement
{
	let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
	
	let profile = appData.read().user.battleNet.starcraft2
		.clone()
		.unwrap_or_default();
	
	let avatarPath = getImagePath(&FileLocation
	{
		fileName: jpgAlt!(Starcraft2::AvatarPrefix, profile.id),
		group: Path_Avatars.into(),
		platform: BattleNetApi::Platform.to_lowercase(),
	});
	
	return rect()
		.cross_align(Alignment::Center)
		.direction(Direction::Vertical)
		.spacing(10.0)
		.width(Size::percent(100.0))
		
		.child(
			rect()
				.direction(Direction::Horizontal)
				.main_align(Alignment::Center)
				.spacing(10.0)
				.width(Size::percent(100.0))
				
				.maybe_child(avatarPath.is_some().then(||
					ImageViewer::new(PathBuf::from(avatarPath.unwrap()))
						.width(Size::px(64.0))
				))
				
				.child(profile.name)
				.child(format!("({})", profile.region.as_ref()))
		)
		
		.child(
			rect()
				.direction(Direction::Horizontal)
				.main_align(Alignment::Center)
				.spacing(10.0)
				.width(Size::percent(100.0))
				
				.child("Total Achievement Points:")
				.child(profile.totalAchievementPoints.to_string())
		)
		
		.child(
			rect()
				.direction(Direction::Horizontal)
				.main_align(Alignment::Center)
				.spacing(10.0)
				
				.child("Total Swarm Level:")
				.child(profile.totalSwarmLevel.to_string())
		);
}
