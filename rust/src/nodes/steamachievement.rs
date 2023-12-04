use ::godot::bind::{GodotClass, godot_api};
use ::godot::builtin::GString;
use ::godot::engine::{NodeExt, Label, HBoxContainer, IHBoxContainer, Image, ImageTexture, ProjectSettings, TextureRect};
use ::godot::obj::Base;
use crate::{achievementIcon, dateFormat};
use crate::data::SteamAchievement;
use crate::platforms::Platform;

const PathDescription: &'static str = "%Description";
const PathGlobalPercentage: &'static str = "%GlobalPercentage";
const PathIcon: &'static str = "%Icon";
const PathLabel: &'static str = "%Label";
const PathUnlockTime: &'static str = "%UnlockTime";

#[derive(GodotClass)]
#[class(base=HBoxContainer)]
pub struct SteamAchievementNode
{
	#[base]
	base: Base<HBoxContainer>,
	
	pub appId: u32,
	pub achievement: SteamAchievement,
	
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
}

impl SteamAchievementNode
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
		
		self.description = self.achievement.description.to_owned().into();
		self.iconPath = achievementIcon!(Platform::nameOf(Platform::Steam).to_lowercase(), self.appId, self.achievement.id).into();
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
impl IHBoxContainer for SteamAchievementNode
{
	fn init(base: Base<HBoxContainer>) -> Self
	{
		return Self
		{
			base,
			appId: 0,
			achievement: SteamAchievement::default(),
			description: GString::default(),
			iconPath: GString::default(),
			label: GString::default(),
			globalPercentage: GString::default(),
			unlockTime: GString::default(),
		};
	}
	
	fn ready(&mut self)
	{
		self.updateData();
	}
}