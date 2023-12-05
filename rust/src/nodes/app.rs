use ::godot::bind::{GodotClass, godot_api};
use godot::engine::global::Side;
use ::godot::engine::{IMarginContainer, MarginContainer, NodeExt, PackedScene, PackedSceneExt, VBoxContainer, load};
use ::godot::obj::Base;
use super::appdata::AppData;
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

impl App
{
	fn generateGameNodes(&mut self)
	{
		let scene = load::<PackedScene>(GameScene);
		if scene.can_instantiate()
		{
			let list = self.base.get_node_as::<AppData>(AppDataPath)
				.bind()
				.user.games.clone();
			
			let mut games = self.base.get_node_as::<VBoxContainer>(GamesPath);
			for game in list
			{
				let mut node = scene.instantiate_as::<GameNode>();
				games.add_child(node.clone().upcast());
				
				let mut gameNode = node.bind_mut();
				gameNode.setGame(game);
				gameNode.refreshAchievements();
			}
		}
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
		
		self.generateGameNodes();
	}
}
