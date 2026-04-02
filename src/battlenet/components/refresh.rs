use freya::prelude::spawn;
use tracing::{info, warn};
use crate::jpgAlt;
use crate::battlenet::BattleNetSettings;
use crate::battlenet::platform::api::BattleNetApi;
use crate::battlenet::platform::starcraft2::Starcraft2;
use crate::data::AppData;
use crate::data::secure::getBattleNetSession;
use crate::io::{Path_Avatars, saveUserData_BattleNet};
use crate::net::limiter::request::{BattleNetOperation, DataOperation, DataOperationResult, FileLocation, DataRequest};

pub async fn handleDataOperation(appData: AppData, operation: BattleNetOperation) -> Option<DataOperationResult>
{
	return match operation
	{
		BattleNetOperation::GetUserInfo => {
			let result = refreshUserInfo(appData).await;
			
			Some(result)
		}
		
		BattleNetOperation::SaveToFile => {
			match saveUserData_BattleNet(&appData.user.battleNet)
			{
				Err(e) => warn!("[BattleNet] Error saving user data: {:?}", e),
				Ok(_) => info!("[BattleNet] Saved user data"),
			}
			
			None
		}
		
		BattleNetOperation::GetSc2PlayerAccount => {
			let result = refreshSc2PlayerAccount(appData).await;
			
			Some(result)
		}
	};
}

pub fn openBrowserForAuthorization(settings: BattleNetSettings)
{
	spawn(async move {
		let api = BattleNetApi::new(settings);
		_ = api.authorize().await;
	});
}

async fn refreshSc2PlayerAccount(mut appData: AppData) -> DataOperationResult
{
	let mut requests = vec![];
	
	let api = BattleNetApi::new(appData.platform.battleNet.clone());
	if let Ok(session) = getBattleNetSession()
	{
		match Starcraft2::accountPlayer(
			&api,
			session,
			match appData.user.battleNet.starcraft2.clone()
			{
				None => appData.platform.battleNet.defaultRegion,
				Some(profile) => profile.region,
			},
			appData.user.battleNet.accountId
		).await
		{
			Err(e) => warn!("[BattleNet] Error refreshing StarCraft 2 account data: {:?}", e),
			
			Ok(payload) => {
				let avatarUrl = payload.avatarUrl.clone();
				if appData.user.battleNet.starcraft2.is_none()
				{
					appData.user.battleNet.starcraft2 = Some(payload.into());
				}
				else if let Some(profile) = appData.user.battleNet.starcraft2.as_mut()
				{
					profile.updateAccount(payload);
				}
				
				// Cache the profile avatar
				if let Some(profile) = appData.user.battleNet.starcraft2.clone()
				{
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: jpgAlt!(Starcraft2::AvatarPrefix, profile.id),
							group: Path_Avatars.into(),
							platform: BattleNetApi::Platform.to_lowercase(),
						}),
						operation: DataOperation::CacheImage,
						url: Some(avatarUrl)
					});
					
				}
			}
		}
	}
	
	return DataOperationResult
	{
		appData,
		requests,
	};
}

async fn refreshUserInfo(mut appData: AppData) -> DataOperationResult
{
	let api = BattleNetApi::new(appData.platform.battleNet.clone());
	if let Ok(session) = getBattleNetSession()
	{
		match api.getUserInfo(session).await
		{
			Err(e) => warn!("[BattleNet] Error refreshing user info: {:?}", e),
			
			Ok(userInfo) => {
				appData.user.battleNet.updateUserInfo(userInfo);
				info!("[BattleNet] User info refreshed");
			}
		}
	}
	
	return appData.into();
}
