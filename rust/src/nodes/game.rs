use ::godot::bind::{GodotClass, godot_api};
use godot::builtin::GString;
use ::godot::engine::{IMarginContainer, Label, MarginContainer, NodeExt, PackedScene, PackedSceneExt, TabContainer, VBoxContainer, load};
use ::godot::obj::Base;
use ::godot::obj::Gd;
use crate::data::{Game, RetroAchievement, SteamAchievement};
use crate::platforms::Platform;
use super::freeChildren;
use super::retroachievement::RetroAchievementNode;
use super::steamachievement::SteamAchievementNode;

const RetroAchievementScene: &'static str = "res://nodes/RetroAchievement.tscn";
const SteamAchievementScene: &'static str = "res://nodes/SteamAchievement.tscn";
const TabScene: &'static str = "res://nodes/PlatformTab.tscn";

const Tabs: &'static str = "%Tabs";
const Title: &'static str = "%Title";

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct GameNode
{
	#[base]
	base: Base<MarginContainer>,
	
	game: Game,
}

impl GameNode
{
	pub fn setGame(&mut self, game: Game)
	{
		self.game = game.clone();
	}
	
	pub fn refreshAchievements(&mut self)
	{
		self.base.get_node_as::<Label>(Title)
			.set_text(self.game.name.to_owned().into());
		
		self.regenerateNodes();
	}
	
	pub fn regenerateNodes(&mut self)
	{
		if let Some(node) = self.base.get_node(Tabs.into()).as_mut()
		{
			let tabs = node.clone().cast::<TabContainer>();
			freeChildren(&mut tabs.clone().upcast());
			
			let tabScene = load::<PackedScene>(TabScene);
			if tabScene.can_instantiate()
			{
				self.generateRetroList(&mut tabs.clone(), tabScene.clone());
				self.generateSteamList(&mut tabs.clone(), tabScene.clone());
			}
		}
	}
	
	fn generateRetroAchievementNode(&mut self, appId: &GString, achievement: &RetroAchievement, listNode: &mut Gd<VBoxContainer>, nodeScene: Gd<PackedScene>)
	{
		let mut node = nodeScene.instantiate_as::<RetroAchievementNode>();
		listNode.add_child(node.clone().upcast());
		
		let mut ra = node.bind_mut();
		ra.appId = appId.to_owned();
		ra.achievement = achievement.to_owned();
		ra.updateData();
	}
	
	fn generateSteamAchievementNode(&mut self, appId: i64, achievement: &SteamAchievement, listNode: &mut Gd<VBoxContainer>, nodeScene: Gd<PackedScene>)
	{
		let mut node = nodeScene.instantiate_as::<SteamAchievementNode>();
		listNode.add_child(node.clone().upcast());
		
		let mut sa = node.bind_mut();
		sa.appId = appId;
		sa.achievement = achievement.to_owned();
		sa.updateData();
	}
	
	fn generateRetroList(&mut self, tabs: &mut Gd<TabContainer>, tabScene: Gd<PackedScene>)
	{
		let nodeScene = load::<PackedScene>(RetroAchievementScene);
		if nodeScene.can_instantiate()
		{
			if let Some(retro) = &self.game.retroAchievements
			{
				let mut tab = tabScene.instantiate_as::<MarginContainer>();
				tab.set_name(Platform::nameOf(Platform::RetroAchievements).into());
				
				if let Some(middle) = tab.get_child(0)
				{
					if let Some(node) = middle.get_child(0)
					{
						let appId = retro.info.id.to_owned().into();
						let mut listNode = node.cast::<VBoxContainer>();
						for achievement in retro.achievements.clone().iter()
						{
							self.generateRetroAchievementNode(&appId, achievement, &mut listNode, nodeScene.clone());
						}
					}
				}
				
				tabs.add_child(tab.clone().upcast());
			}
		}
	}
	
	fn generateSteamList(&mut self, tabs: &mut Gd<TabContainer>, tabScene: Gd<PackedScene>)
	{
		let nodeScene = load::<PackedScene>(SteamAchievementScene);
		if nodeScene.can_instantiate()
		{
			if let Some(steam) = &self.game.steam
			{
				let mut tab = tabScene.instantiate_as::<MarginContainer>();
				tab.set_name(Platform::nameOf(Platform::Steam).into());
				
				if let Some(middle) = tab.get_child(0)
				{
					if let Some(node) = middle.get_child(0)
					{
						let appId = steam.info.id;
						let mut listNode = node.cast::<VBoxContainer>();
						for achievement in steam.achievements.clone().iter()
						{
							self.generateSteamAchievementNode(appId, achievement, &mut listNode, nodeScene.clone());
						}
					}
				}
				
				tabs.add_child(tab.clone().upcast());
			}
		}
	}
}

#[godot_api]
impl IMarginContainer for GameNode
{
	fn init(base: Base<MarginContainer>) -> Self
	{
		return Self
		{
			base,
			game: Game::default(),
		};
	}
	
	fn ready(&mut self)
	{
		self.refreshAchievements();
	}
}
