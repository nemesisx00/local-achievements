use ::godot::bind::{GodotClass, godot_api};
use ::godot::builtin::GString;
use ::godot::engine::{NodeExt, Label, HBoxContainer, IHBoxContainer, Image, ImageTexture, ProjectSettings, TextureRect};
use ::godot::obj::Base;
use crate::data::{RetroAchievement, RetroMode};
use crate::platforms::Platform;
use crate::{dateFormat, achievementIcon};

const PathDescription: &'static str = "%Description";
const PathGlobalPercentage: &'static str = "%GlobalPercentage";
const PathIcon: &'static str = "%Icon";
const PathLabel: &'static str = "%Label";
const PathUnlockTime: &'static str = "%UnlockTime";
const PathHardcorePoints: &'static str = "%HardcorePoints";
const PathSoftcorePoints: &'static str = "%SoftcorePoints";

#[derive(GodotClass)]
#[class(base=HBoxContainer)]
pub struct RetroAchievementNode
{
	#[base]
	base: Base<HBoxContainer>,
	
	pub appId: u32,
	pub achievement: RetroAchievement,
	
	#[export]
	description: GString,
	
	#[export]
	iconPath: GString,
	
	#[export]
	label: GString,
	
	#[export]
	globalPercentage: GString,
	
	#[export]
	unlockTime: GString,
	
	#[export]
	pointsHardcore: GString,
	
	#[export]
	pointsSoftcore: GString,
}

impl RetroAchievementNode
{
	pub fn updateData(&mut self)
	{
		self.globalPercentage = match self.achievement.globalPercentage
		{
			Some(gp) => format!("{}%", gp).into(),
			None => GString::default(),
		};
		
		self.unlockTime = match self.achievement.timestamp
		{
			Some(ts) => dateFormat!(ts).into(),
			None => GString::default(),
		};
		
		self.pointsHardcore = match &self.achievement.points
		{
			Some(map) => match map.iter().find(|(k, _)| RetroMode::Hardcore == **k)
			{
				Some((_, pts)) => pts.to_string().into(),
				None => GString::default(),
			},
			None => GString::default(),
		};
		
		self.pointsSoftcore = match &self.achievement.points
		{
			Some(map) => match map.iter().find(|(k, _)| RetroMode::Softcore == **k)
			{
				Some((_, pts)) => pts.to_string().into(),
				None => GString::default(),
			},
			None => GString::default(),
		};
		
		self.description = self.achievement.description.to_owned().into();
		self.iconPath = achievementIcon!(Platform::nameOf(Platform::RetroAchievements).to_lowercase(), self.appId, self.achievement.id).into();
		self.label = self.achievement.name.to_owned().into();
		
		self.refresh();
	}
	
	fn loadIcon(&mut self, resourcePath: GString)
	{
		let path = ProjectSettings::singleton().globalize_path(resourcePath);
		if let Some(image) = Image::load_from_file(path)
		{
			if let Some(imageTexture) = ImageTexture::create_from_image(image)
			{
				self.base.get_node_as::<TextureRect>(PathIcon)
					.set_texture(imageTexture.upcast());
			}
		}
	}
	
	fn refresh(&mut self)
	{
		self.setLabelText(PathDescription, self.description.to_owned());
		self.setLabelText(PathLabel, self.label.to_owned());
		self.setLabelText(PathHardcorePoints, self.pointsHardcore.to_owned());
		self.setLabelText(PathSoftcorePoints, self.pointsSoftcore.to_owned());
		self.setLabelText(PathGlobalPercentage, self.globalPercentage.to_owned());
		self.setLabelText(PathUnlockTime, self.unlockTime.to_owned());
		self.loadIcon(self.iconPath.to_owned());
	}
	
	fn setLabelText(&mut self, path: &str, value: GString)
	{
		self.base.get_node_as::<Label>(path)
			.set_text(value);
	}
}

#[godot_api]
impl IHBoxContainer for RetroAchievementNode
{
	fn init(base: Base<HBoxContainer>) -> Self
	{
		return Self
		{
			base,
			appId: 0,
			achievement: RetroAchievement::default(),
			description: GString::default(),
			iconPath: GString::default(),
			label: GString::default(),
			pointsHardcore: GString::default(),
			pointsSoftcore: GString::default(),
			globalPercentage: GString::default(),
			unlockTime: GString::default(),
		};
	}
	
	fn ready(&mut self)
	{
		self.updateData();
		self.refresh();
	}
}
