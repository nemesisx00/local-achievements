use ::godot::bind::{GodotClass, godot_api};
use ::godot::engine::{IMarginContainer, Label, MarginContainer, NodeExt, PackedScene, PackedSceneExt, TabContainer, VBoxContainer, load};
use ::godot::obj::Base;
use ::godot::obj::Gd;
use crate::data::{Game, RetroAchievement, SteamAchievement};
use crate::io::loadUserData;
use crate::platforms::Platform;
use super::freeChildren;
use super::retroachievement::RetroAchievementNode;
use super::steamachievement::SteamAchievementNode;

const RetroAchievementScene: &'static str = "res://nodes/RetroAchievement.tscn";
const SteamAchievementScene: &'static str = "res://nodes/SteamAchievement.tscn";
const GameScene: &'static str = "res://nodes/Game.tscn";
const TabScene: &'static str = "res://nodes/PlatformTab.tscn";

const Tabs: &'static str = "%Tabs";
const Title: &'static str = "%Title";

#[derive(Debug, GodotClass)]
#[class(base=MarginContainer)]
pub struct GameNode
{
	#[base]
	base: Base<MarginContainer>,
	
	#[export]
	appId: u32,
	
	game: Game,
}

impl GameNode
{
	pub fn refreshAchievements(&mut self)
	{
		if self.appId > 0
		{
			if let Ok(user) = loadUserData()
			{
				if let Some(game) = user.games.iter()
					.find(|g| g.steam.clone().is_some_and(|s| s.info.id == self.appId as usize))
				{
					self.base.get_node_as::<Label>(Title)
						.set_text(game.name.to_owned().into());
					
					self.game = game.clone();
					self.regenerateNodes();
				}
			}
		}
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
	
	fn generateRetroAchievementNode(&mut self, achievement: &RetroAchievement, listNode: &mut Gd<VBoxContainer>, nodeScene: Gd<PackedScene>)
	{
		let mut node = nodeScene.instantiate_as::<RetroAchievementNode>();
		listNode.add_child(node.clone().upcast());
		
		let mut ra = node.bind_mut();
		ra.appId = self.appId;
		ra.achievement = achievement.to_owned();
		ra.updateData();
	}
	
	fn generateSteamAchievementNode(&mut self, achievement: &SteamAchievement, listNode: &mut Gd<VBoxContainer>, nodeScene: Gd<PackedScene>)
	{
		let mut node = nodeScene.instantiate_as::<SteamAchievementNode>();
		listNode.add_child(node.clone().upcast());
		
		let mut sa = node.bind_mut();
		sa.appId = self.appId;
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
						let mut listNode = node.cast::<VBoxContainer>();
						for achievement in retro.achievements.clone().iter()
						{
							self.generateRetroAchievementNode(achievement, &mut listNode, nodeScene.clone());
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
						let mut listNode = node.cast::<VBoxContainer>();
						for achievement in steam.achievements.clone().iter()
						{
							self.generateSteamAchievementNode(achievement, &mut listNode, nodeScene.clone());
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
			appId: 0,
		};
	}
	
	fn ready(&mut self)
	{
		self.refreshAchievements();
	}
}
