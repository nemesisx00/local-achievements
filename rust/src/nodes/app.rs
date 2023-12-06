use ::godot::bind::{GodotClass, godot_api};
use ::godot::builtin::Variant;
use ::godot::builtin::meta::FromGodot;
use ::godot::engine::{IMarginContainer, MarginContainer, NodeExt, PackedScene, PackedSceneExt, VBoxContainer, load};
use ::godot::engine::global::Side;
use ::godot::obj::{Base, WithBaseField};
use crate::data::User;
use super::game::GameNode;

const AppDataPath: &'static str = "%AppData";
const GameScene: &'static str = "res://nodes/Game.tscn";
const GamesPath: &'static str = "%Games";
const MarginSize: f32 = 25.0;

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct App
{
	#[base]
	base: Base<MarginContainer>,
}

#[godot_api]
impl App
{
	fn generateGameNodes(&self, user: User)
	{
		let scene = load::<PackedScene>(GameScene);
		if scene.can_instantiate()
		{
			let mut games = self.base.get_node_as::<VBoxContainer>(GamesPath);
			for game in user.games
			{
				let mut node = scene.instantiate_as::<GameNode>();
				
				let mut gameNode = node.bind_mut();
				gameNode.setGame(game);
				
				games.add_child(gameNode.to_gd().upcast());
			}
		}
	}
	
	#[func]
	pub fn handleDataLoaded(&mut self, user: Variant)
	{
		let user = User::from_variant(&user);
		self.generateGameNodes(user);
	}
}

#[godot_api]
impl IMarginContainer for App
{
	fn init(base: Base<MarginContainer>) -> Self
	{
		return Self
		{
			base,
		};
	}
	
	fn ready(&mut self)
	{
		self.base.set_offset(Side::SIDE_LEFT, MarginSize);
		self.base.set_offset(Side::SIDE_TOP, MarginSize);
		self.base.set_offset(Side::SIDE_RIGHT, MarginSize);
		self.base.set_offset(Side::SIDE_BOTTOM, MarginSize);
	}
}
