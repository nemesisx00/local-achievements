use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Copy, Default, Debug, Deserialize, Display, EnumIter, EnumString, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Language
{
	#[default]
	English
}
