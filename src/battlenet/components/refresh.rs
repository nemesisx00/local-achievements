use freya::prelude::spawn;
use tracing::{info, warn};
use crate::battlenet::BattleNetSettings;
use crate::battlenet::platform::api::BattleNetApi;
use crate::data::AppData;
use crate::io::saveUserData_BattleNet;
use crate::net::limiter::request::{BattleNetOperation, DataOperationResult};

pub async fn handleDataOperation(appData: AppData, operation: BattleNetOperation) -> Option<DataOperationResult>
{
	return match operation
	{
		BattleNetOperation::SaveToFile => {
			match saveUserData_BattleNet(&appData.user.battleNet)
			{
				Err(e) => warn!("[BattleNet] Error saving user data: {:?}", e),
				Ok(_) => info!("[BattleNet] Saved user data"),
			}
			None
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
