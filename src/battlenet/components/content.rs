use freya::prelude::{Alignment, ChildrenExt, Component, ContainerSizeExt, ContainerWithContentExt, Direction, IntoElement, Size, label, rect};
use freya::radio::use_radio;
use crate::data::AppData;
use crate::data::radio::{AppDataChannel, DataChannel};
use crate::net::limiter::RateLimiter;

#[derive(Clone, PartialEq)]
pub struct BattleNetContentElement {}

impl Component for BattleNetContentElement
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		
		/*
		use_hook(|| if platformState.read().session.battleNet.clone()
				.is_some_and(|s: BattleNetSession| s.validate())
			&& userState.read().battleNet.battleTag.is_empty()
		{
			refresh();
		});
		*/
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.spacing(10.0)
			.width(Size::Fill)
			
			.child(
				label()
					.text(format!("BattleTag: {}", appData.read().user.battleNet.battleTag))
			);
	}
}

impl BattleNetContentElement
{
	pub fn new() -> Self
	{
		return Self {};
	}
}

pub fn refresh()
{
}

pub fn startNewSession()
{
}
/*
	spawn(async move {
		if let Some(session) = BattleNetSessionData()
		{
			let api = Api::withSession(BattleNetAuthData(), session);
			
			match api.userInfo().await
			{
				Err(e) => warn!("Error loading user info (Battle.Net): {:?}", e),
				Ok(userInfo) => {
					BattleNetUserData.write().accountId = userInfo.id;
					BattleNetUserData.write().battleTag = userInfo.battletag.clone();
				}
			}
			
			match Starcraft2::accountPlayer(&api, BattleNetUserData().accountId).await
			{
				Err(e) => warn!("Error loading StarCraft 2 Player metadata (Battle.Net): {:?}", e),
				Ok(player) => {
					BattleNetUserData.write().profileId = player.profileId;
					BattleNetUserData.write().region = Region::from(player.realmId, player.regionId);
					info!("StarCraft 2 Player metadata retrieved (Battle.Net)!");
				}
			}
			
			match Starcraft2::profileStatic(&api, BattleNetUserData().region).await
			{
				Err(e) => warn!("Error loading StarCraft 2 Static Profile data (Battle.Net): {:?}", e),
				Ok(profile) => {
					println!("Number of achievements returned: {}", profile.achievements.len());
				},
			}
			
			NotificationList.write().push_back("Battle.Net Data Refreshed".into());
			
			match saveUserData_BattleNet(&BattleNetUserData())
			{
				Err(e) => println!("Error saving user data (Battle.Net): {:?}", e),
				Ok(_) => println!("Saved user data (Battle.Net)"),
			}
		}
	});
}

pub fn startNewSession()
{
	spawn(async move {
		let mut api = Api::new(BattleNetAuthData());
		match api.authorize().await
		{
			Err(e) => error!("Failed Battle.Net authorization flow: {:?}", e),
			Ok(session) => {
				info!("Battle.Net authorization flow succeeded");
				*BattleNetSessionData.write() = Some(session);
				NotificationList.write().push_back("Battle.Net Authorized".into());
			},
		}
	});
}
*/
