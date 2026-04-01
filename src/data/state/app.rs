use serde::{Deserialize, Serialize};
use crate::data::AppSettings;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppState
{
	pub settings: AppSettings,
}
