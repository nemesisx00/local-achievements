use freya::prelude::spawn;
use crate::battlenet::BattleNetSettings;
use crate::battlenet::platform::api::BattleNetApi;

pub fn openBrowserForAuthorization(settings: BattleNetSettings)
{
	spawn(async move {
		let api = BattleNetApi::new(settings);
		_ = api.authorize().await;
	});
}
