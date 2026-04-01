use serde::{Deserialize, Serialize};
//use crate::egs::EgsSettings;
use crate::rpcs3::Rpcs3Settings;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SettingsState
{
	//pub egs: EgsSettings,
	pub rpcs3: Rpcs3Settings,
}
