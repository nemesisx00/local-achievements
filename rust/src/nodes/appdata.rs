use ::godot::bind::{GodotClass, godot_api};
use ::godot::builtin::meta::ToGodot;
use ::godot::engine::{INode, Node};
use ::godot::log::godot_print;
use ::godot::obj::{Base, WithBaseField};
use crate::data::{User, Game};
use crate::io::{loadUserData, saveUserData};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AppData
{
	#[base]
	base: Base<Node>,
	
	#[var]
	pub user: User,
}

#[godot_api]
impl AppData
{
	pub const SignalDataLoaded: &'static str = "DataLoaded";
	
	#[signal]
	pub fn DataLoaded(user: User);
	
	/**
	Retrieve a `crate::data::Game` from the games list by matching the given
	identifier against the title or one of the platform-specific IDs.
	
	Parameter | Description
	---|---
	identifier | The value used to identify the desired game. Attempts to match the title first, then any of the platform-specific ID values. Exact matches only.
	*/
	pub fn getGame(&self, identifier: String) -> Option<Game>
	{
		let game = self.user.games.iter()
			.find(|g| g.name == identifier || g.getIds().values().any(|v| v == &identifier))?;
		
		return Some(game.to_owned());
	}
	
	pub fn reloadData(&mut self)
	{
		let user = match loadUserData()
		{
			Ok(data) => data,
			Err(_) => User::default(),
		};
		
		self.user = user;
		
		self.to_gd().emit_signal(Self::SignalDataLoaded.into(), &[self.user.to_variant()]);
	}
	
	pub fn saveData(&mut self)
	{
		match saveUserData(self.user.to_owned())
		{
			Ok(()) => godot_print!("User data saved!"),
			Err(e) => godot_print!("Failed saving the user data: {:?}", e),
		}
	}
}

#[godot_api]
impl INode for AppData
{
	fn init(base: Base<Node>) -> Self
	{
		return Self
		{
			base,
			user: User::default(),
		}
	}
	
	fn ready(&mut self)
	{
		self.reloadData();
	}
}
