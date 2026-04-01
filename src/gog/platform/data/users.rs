use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserInfo
{
	pub avatar: Avatars,
	pub created_date: String,
	pub id: String,
	pub is_employee: bool,
	pub username: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Avatars
{
	pub gog_image_id: String,
	pub large: String,
	pub large_2x: String,
	pub medium: String,
	pub medium_2x: String,
	pub sdk_img_32: String,
	pub sdk_img_64: String,
	pub sdk_img_184: String,
	pub small: String,
	pub small_2x: String,
}
