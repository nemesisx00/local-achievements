use ::godot::bind::{GodotClass, godot_api};
use ::godot::engine::{IVBoxContainer, Label, MarginContainer, NodeExt, PackedScene, PackedSceneExt, TabContainer, VBoxContainer, load};
use ::godot::obj::Base;
use ::godot::obj::Gd;
use crate::{achievementIcon, dateFormat};
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

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct GameNode
{
	#[base]
	base: Base<VBoxContainer>,
	
	#[export]
	appId: u32,
	
	game: Game,
	sceneRetroAchievement: Gd<PackedScene>,
	sceneSteamAchievement: Gd<PackedScene>,
	sceneTab: Gd<PackedScene>,
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
					self.base.get_node_as::<Label>("%Title")
						.set_text(game.name.to_owned().into());
					
					self.game = game.clone();
					self.regenerateNodes();
				}
			}
		}
	}
	
	pub fn regenerateNodes(&mut self)
	{
		let mut tabs = self.base.get_node(Tabs.into())
			.expect(format!("Failed to find the TabContainer with path '{}'", Tabs).as_str())
			.cast::<TabContainer>();
		
		freeChildren(&mut tabs.clone().upcast());
		self.generateRetroList(&mut tabs);
		self.generateSteamList(&mut tabs);
	}
	
	fn generateAchievementNode_Retro(&mut self, achievement: &RetroAchievement, listNode: &mut Gd<VBoxContainer>)
	{
		let mut node = self.sceneRetroAchievement.instantiate_as::<RetroAchievementNode>();
		listNode.add_child(node.clone().upcast());
		
		let mut ra = node.bind_mut();
		ra.appId = self.appId;
		ra.achievement = achievement.to_owned();
		ra.updateData();
	}
	
	fn generateAchievementNode_Steam(&mut self, achievement: &SteamAchievement, listNode: &mut Gd<VBoxContainer>)
	{
		let mut node = self.sceneSteamAchievement.instantiate_as::<SteamAchievementNode>();
		let globalPercentage = match achievement.globalPercentage
		{
			Some(gp) => gp,
			None => -1.0,
		};
		
		let unlockTime = match achievement.timestamp
		{
			Some(ts) => dateFormat!(ts),
			None => String::default(),
		};
		
		node.bind_mut()
			.updateData(
				achievement.description.to_owned().into(),
				achievementIcon!(Platform::nameOf(Platform::Steam).to_lowercase(), self.appId, achievement.id).into(),
				achievement.name.to_owned().into(),
				globalPercentage,
				unlockTime.into()
			);
		
		listNode.add_child(node.clone().upcast());
	}
	
	fn generateRetroList(&mut self, tabs: &mut Gd<TabContainer>)
	{
		if self.sceneRetroAchievement.can_instantiate()
		{
			if let Some(retro) = &self.game.retroAchievements
			{
				let mut tab = self.sceneTab.instantiate_as::<MarginContainer>();
				tab.set_name(Platform::nameOf(Platform::RetroAchievements).into());
				
				if let Some(middle) = tab.get_child(0)
				{
					if let Some(node) = middle.get_child(0)
					{
						let mut listNode = node.cast::<VBoxContainer>();
						for achievement in retro.achievements.clone().iter()
						{
							self.generateAchievementNode_Retro(achievement, &mut listNode);
						}
					}
				}
				
				tabs.add_child(tab.clone().upcast());
			}
		}
	}
	
	fn generateSteamList(&mut self, tabs: &mut Gd<TabContainer>)
	{
		if self.sceneSteamAchievement.can_instantiate()
		{
			if let Some(steam) = &self.game.steam
			{
				let mut tab = self.sceneTab.instantiate_as::<MarginContainer>();
				tab.set_name(Platform::nameOf(Platform::Steam).into());
				
				if let Some(middle) = tab.get_child(0)
				{
					if let Some(node) = middle.get_child(0)
					{
						let mut listNode = node.cast::<VBoxContainer>();
						for achievement in steam.achievements.clone().iter()
						{
							self.generateAchievementNode_Steam(achievement, &mut listNode);
						}
					}
				}
				
				tabs.add_child(tab.clone().upcast());
			}
		}
	}
}

#[godot_api]
impl IVBoxContainer for GameNode
{
	fn init(base: Base<VBoxContainer>) -> Self
	{
		return Self
		{
			base,
			game: Game::default(),
			appId: 0,
			sceneRetroAchievement: PackedScene::new(),
			sceneSteamAchievement: PackedScene::new(),
			sceneTab: PackedScene::new(),
		};
	}
	
	fn ready(&mut self)
	{
		self.sceneRetroAchievement = load::<PackedScene>(RetroAchievementScene);
		self.sceneSteamAchievement = load::<PackedScene>(SteamAchievementScene);
		self.sceneTab = load::<PackedScene>(TabScene);
		
		self.refreshAchievements();
	}
}
