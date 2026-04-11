use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Avatars
{
	pub large: String,
	pub medium: String,
	pub small: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Rarity
{
	pub percent: f64,
}
