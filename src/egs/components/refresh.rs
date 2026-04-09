use tracing::{info, warn};
use crate::egs::platform::data::private::KeyImageType;
use crate::{join, jpg, png};
use crate::data::AppData;
use crate::data::secure::getEpicGamesStoreAccountId;
use crate::egs::EgsApi;
use crate::io::{FileName_GameIcon, Path_Avatars, Path_Games, saveUserData_EpicGamesStore};
use crate::net::limiter::request::{DataOperation, DataOperationResult,
	DataRequest, EpicGamesStoreOperation, FileLocation};

pub async fn handleDataOperation(appData: AppData, operation: EpicGamesStoreOperation) -> Option<DataOperationResult>
{
	return match operation
	{
		EpicGamesStoreOperation::GetAchievementProgress(productId) => {
			let result = refreshEgsAchievementProgress(appData, &productId);
			info!("[EpicGamesStore] Refreshed achievements progress for {}", productId);
			
			Some(result)
		}
		
		EpicGamesStoreOperation::GetAchievementsList(sandboxId) => {
			let result = refreshEgsAchievementMetadata(appData, &sandboxId);
			info!("[EpicGamesStore] Refreshed achievements metadata for {}", sandboxId);
			
			Some(result)
		}
		
		EpicGamesStoreOperation::GetPlayerProfile => {
			let result = refreshEgsPlayerProfile(appData);
			info!("[EpicGamesStore] Refreshed user profile data");
			
			Some(result)
		}
		
		EpicGamesStoreOperation::GetPlayerProfilePrivate => {
			let result = refreshEgsPlayerProfilePrivate(appData);
			info!("[EpicGamesStore] Refreshed user game list data");
			
			Some(result)
		}
		
		EpicGamesStoreOperation::SaveToFile => {
			match saveUserData_EpicGamesStore(&appData.user.egs)
			{
				Err(e) => warn!("[EpicGamesStore] Error saving user data: {:?}", e),
				Ok(_) => info!("[EpicGamesStore] Saved user data"),
			}
		
			None
		}
	};
}

fn refreshEgsAchievementMetadata(mut appData: AppData, sandboxId: &String) -> DataOperationResult
{
	let mut requests = vec![];
	
	if let Ok(accountId) = getEpicGamesStoreAccountId()
	{
		match EgsApi::getAchievementMetadata(&accountId, sandboxId)
		{
			Err(e) => warn!("Failed to refresh Epic Games Store achievements metadata data: {:?}", e),
			Ok(payload) => {
				
				// Cache icons
				let group = join!(Path_Games, sandboxId.clone());
				let platform = EgsApi::Platform.to_lowercase();
				
				for container in payload.data.Achievement
					.productAchievementsRecordBySandbox.achievements.iter()
				{
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: container.achievement.unlockedIconId.clone(),
							group: group.clone(),
							platform: platform.clone(),
						}),
						operation: DataOperation::CacheImage(false),
						url: Some(container.achievement.unlockedIconLink.clone()),
					});
					
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: container.achievement.lockedIconId.clone(),
							group: group.clone(),
							platform: platform.clone(),
						}),
						operation: DataOperation::CacheImage(false),
						url: Some(container.achievement.lockedIconLink.clone()),
					});
					
					if let Some(game) = appData.user.egs.games.iter_mut()
						.find(|g| &g.sandboxId == sandboxId)
					{
						match game.achievements.iter_mut()
							.find(|a| a.id == container.achievement.name)
						{
							None => game.achievements.push((&container.achievement).into()),
							Some(chievo) => chievo.updateMetadata(&container.achievement),
						}
					}
				}
				
				if let Some(game) = appData.user.egs.games.iter_mut()
					.find(|g| &g.sandboxId == sandboxId)
				{
					game.achievementsCount = payload.data.Achievement.productAchievementsRecordBySandbox.totalAchievements;
					game.maxXp = payload.data.Achievement.productAchievementsRecordBySandbox.totalProductXP;
					game.platinumRarity = (payload.data.Achievement.productAchievementsRecordBySandbox.platinumRarity.percent * 10.0) as u64;
					game.productId = payload.data.Achievement.productAchievementsRecordBySandbox.productId.clone();
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

fn refreshEgsAchievementProgress(mut appData: AppData, productId: &String) -> DataOperationResult
{
	if let Ok(accountId) = getEpicGamesStoreAccountId()
	{
		match EgsApi::getAchievementProgress(&accountId, productId)
		{
			Err(e) => warn!("Failed to refresh Epic Games Store achievements progress data: {:?}", e),
			Ok(payload) => {
				if let Some(game) = appData.user.egs.games.iter_mut()
					.find(|g| &g.productId == productId)
				{
					game.currentXp = payload.data.PlayerProfile.playerProfile.productAchievements.data.totalXP;
					game.achievementsUnlocked = payload.data.PlayerProfile.playerProfile.productAchievements.data.totalUnlocked;
				}
				
				for achievement in payload.data.PlayerProfile.playerProfile
					.productAchievements.data.playerAchievements.iter()
				{
					if let Some(game) = appData.user.egs.games.iter_mut()
						.find(|g| &g.productId == productId)
					{
						match game.achievements.iter_mut()
							.find(|a| a.id == achievement.playerAchievement.achievementName)
						{
							None => game.achievements.push(achievement.into()),
							Some(chievo) => chievo.updateProgress(achievement),
						}
					}
				}
			}
		}
	}
	
	return appData.into();
}

fn refreshEgsPlayerProfile(mut appData: AppData) -> DataOperationResult
{
	let mut requests = vec![];
	
	if let Ok(accountId) = getEpicGamesStoreAccountId()
	{
		match EgsApi::getPlayerProfile(&accountId)
		{
			Err(e) => warn!("Failed to refresh Epic Games Store player profile data: {:?}", e),
			Ok(payload) => {
				// Cache icon
				requests.push(DataRequest
				{
					destination: Some(FileLocation
					{
						fileName: png!(accountId),
						group: Path_Avatars.into(),
						platform: EgsApi::Platform.to_lowercase(),
					}),
					operation: DataOperation::CacheImage(true),
					url: Some(payload.data.PlayerProfile.playerProfile.avatar.large.clone()),
				});
				
				appData.user.egs.updateProfile(payload);
			}
		}
	}
	
	return DataOperationResult
	{
		appData,
		requests,
	};
}

fn refreshEgsPlayerProfilePrivate(mut appData: AppData) -> DataOperationResult
{
	let mut requests = vec![];
	
	if let Ok(accountId) = getEpicGamesStoreAccountId()
	{
		match EgsApi::getPlayerProfilePrivate(&accountId)
		{
			Err(e) => warn!("Failed to refresh Epic Games Store private player profile data: {:?}", e),
			Ok(payload) => {
				//Cache game icons
				for summary in payload.data.PlayerProfile.playerProfile.achievementsSummaries.data.iter()
				{
					if let Some(keyImage) = summary.baseOfferForSandbox.keyImages.iter()
						.find(|ki| ki.r#type == KeyImageType::OfferImageWide)
					{
						requests.push(DataRequest
						{
							destination: Some(FileLocation
							{
								fileName: jpg!(FileName_GameIcon),
								group: join!(Path_Games, summary.sandboxId),
								platform: EgsApi::Platform.to_lowercase(),
							}),
							operation: DataOperation::CacheImage(false),
							url: Some(keyImage.url.clone())
						});
					}
				}
				
				appData.user.egs.updateProfilePrivate(payload);
			}
		}
	}
	
	return DataOperationResult
	{
		appData,
		requests,
	};
}
