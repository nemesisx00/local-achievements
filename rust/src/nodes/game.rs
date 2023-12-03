#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::chrono::{Local, LocalResult, NaiveDateTime};
use ::godot::bind::{GodotClass, godot_api};
use ::godot::engine::{IVBoxContainer, Label, NodeExt, PackedScene, PackedSceneExt, VBoxContainer, load};
use ::godot::obj::Base;
use ::godot::obj::Gd;
use crate::achievementIcon;
use crate::data::Achievement as AchievementData;
use crate::io::loadUserData;
use crate::platforms::Platform;
use super::achievement::Achievement;

const AchievementScene: &'static str = "res://nodes/Achievement.tscn";
const GameScene: &'static str = "res://nodes/Game.tscn";

const PathAchievementsList: &'static str = "%AchievementsList";

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct Game
{
	#[base]
	base: Base<VBoxContainer>,
	
	#[export]
	appId: u32,
	
	achievements: Vec<AchievementData>,
	scene: Gd<PackedScene>,
}

impl Game
{
	pub fn refreshAchievements(&mut self)
	{
		if self.appId > 0
		{
			if let Ok(user) = loadUserData()
			{
				if let Some(game) = user.games.iter()
					.find(|g| g.steam.clone().is_some_and(|s| s.id == self.appId as usize))
				{
					self.base.get_node_as::<Label>("%Title")
						.set_text(game.name.to_owned().into());
					
					self.achievements = game.achievements.clone();
					self.regenerateNodes();
				}
			}
		}
	}
	
	pub fn regenerateNodes(&mut self)
	{
		let mut listNode = self.base.get_node(PathAchievementsList.into())
			.expect(format!("Failed to find VBoxContainer with path '{}'", PathAchievementsList).as_str())
			.cast::<VBoxContainer>();
		
		if listNode.get_child_count() > 0
		{
			for i in (0..listNode.get_child_count()).rev()
			{
				if let Some(c) = listNode.get_child(i).as_mut()
				{
					c.queue_free();
				}
			}
		}
		
		if self.scene.can_instantiate()
		{
			for achievement in self.achievements.clone().iter()
			{
				self.generateAchievementNode(achievement, &mut listNode);
			}
		}
	}
	
	fn generateAchievementNode(&mut self, achievement: &AchievementData, listNode: &mut Gd<VBoxContainer>)
	{
		let mut node = self.scene.instantiate_as::<Achievement>();
		
		if let Some(platform) = achievement.platforms.iter().find(|p| p.platform == Platform::Steam)
		{
			let globalPercentage = match platform.globalPercentage
			{
				Some(gp) => gp,
				None => -1.0,
			};
			
			let unlockTime = match platform.timestamp
			{
				Some(ts) => {
					match NaiveDateTime::from_timestamp_millis(ts as i64)
					{
						Some(ndt) => match ndt.and_local_timezone(Local)
						{
							LocalResult::Single(dt) => dt.format("%B %d, %Y %l:%M %p")
								.to_string(),
							_ => String::default(),
						},
						None => String::default(),
					}
				},
				None => String::default(),
			};
			
			node.bind_mut()
				.updateData(
					platform.description.to_owned().into(),
					achievementIcon!(Platform::nameOf(Platform::Steam).to_lowercase(), self.appId, platform.id).into(),
					platform.name.to_owned().into(),
					globalPercentage,
					unlockTime.into()
				);
			
			listNode.add_child(node.clone().upcast());
		}
	}
}

#[godot_api]
impl IVBoxContainer for Game
{
	fn init(base: Base<VBoxContainer>) -> Self
	{
		return Self
		{
			base,
			achievements: Vec::default(),
			appId: 0,
			scene: PackedScene::new(),
		};
	}
	
	fn ready(&mut self)
	{
		self.scene = load::<PackedScene>(AchievementScene);
		self.refreshAchievements();
	}
}
