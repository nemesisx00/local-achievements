use ::godot::builtin::{Dictionary, Variant};
use ::godot::builtin::meta::{ConvertError, FromGodot, GodotConvert, ToGodot};
use ::serde::{Deserialize, Serialize};
use crate::{readVariant, readVariantOption};

/**
Profile information for a Steam user.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamProfile
{
	/// The path to the user's avatar
	pub avatar: Option<String>,
	
	/// The user's 64-bit Steam ID
	pub id: String,
	
	/// The user's current publicly visible display name.
	pub name: String,
}

impl FromGodot for SteamProfile
{
	fn from_godot(via: Self::Via) -> Self
	{
		return Self::fromDict(via);
	}
	
	fn from_variant(variant: &Variant) -> Self
	{
		return Self::fromVariant(variant);
	}
	
	fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_godot(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_variant(variant));
	}
}

impl GodotConvert for SteamProfile
{
	type Via = Dictionary;
}

impl ToGodot for SteamProfile
{
	fn into_godot(self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_godot(&self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_variant(&self) -> Variant
	{
		return self.buildDict().to_variant();
	}
}

impl SteamProfile
{
	pub fn update(&mut self, id: String, name: String, avatar: Option<String>)
	{
		self.id = id;
		self.name = name;
		self.avatar = avatar;
	}
	
	fn buildDict(&self) -> Dictionary
	{
		let mut dict = Dictionary::new();
		dict.insert("id", self.id.to_godot());
		dict.insert("name", self.name.to_godot());
		dict.insert("avatar", match self.avatar.to_owned()
		{
			Some(str) => str,
			None => String::new(),
		}.to_godot());
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let id = readVariant!(dict.get("id"), String);
		let name = readVariant!(dict.get("name"), String);
		let avatar = readVariantOption!(dict.get("avatar"), String);
		
		return Self
		{
			id,
			name,
			avatar,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
	}
}
