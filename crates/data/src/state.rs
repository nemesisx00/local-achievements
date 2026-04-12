use serde::{Deserialize, Serialize};
use crate::settings::AppSettings;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppState
{
	pub settings: AppSettings,
}
