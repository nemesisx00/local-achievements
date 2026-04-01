use tracing::{info, warn};
use crate::data::AppData;
use crate::io::saveUserData_Rpcs3;
use crate::rpcs3::Rpcs3Api;

pub async fn refreshUserData(mut appData: AppData) -> AppData
{
	if !appData.platform.rpcs3.appDataDirectory.is_empty()
	{
		let api = Rpcs3Api::from(appData.platform.rpcs3.clone());
		
		match api.generateGameList()
		{
			Err(e) => warn!("[RPCS3] Error generating game list: {:?}", e),
			
			Ok(games) => {
				for npCommId in games.iter().cloned().map(|g| g.npCommId)
				{
					if let Err(e) = api.cacheGameIcons(&npCommId)
					{
						warn!("[RPCS3] Error caching the icons for {}: {:?}", npCommId, e);
					}
				}
				info!("[RPCS3] Cached game icons");
				//notification "Icons Cached"
				
				appData.user.rpcs3.updateGamesList(games);
				info!("[RPCS3] Refreshed game list");
				//notification "Trophy Data Loaded"
			}
		}
		
		match api.getRpcnId()
		{
			Err(e) => warn!("[RPCS3] Failed to read RPCN ID: {:?}", e),
			
			Ok(rpcnId) => {
				appData.user.rpcs3.name = rpcnId;
				info!("[RPCS3] Refreshed RPCN ID");
				//notification "RPCN ID Loaded"
			},
		}
		
		appData.user.rpcs3.accountId = appData.platform.rpcs3.accountId;
		
		match saveUserData_Rpcs3(&appData.user.rpcs3)
		{
			Err(e) => warn!("[RPCS3] Error saving user data: {:?}", e),
			Ok(_) => info!("[RPCS3] Saved user data"),
		}
	}
	
	return appData;
}
