use ::godot::bind::{GodotClass, godot_api};
use ::godot::builtin::GString;
use ::godot::engine::{NodeExt, Label, HBoxContainer, IHBoxContainer, Image, ImageTexture, ProjectSettings, TextureRect};
use ::godot::obj::Base;

const PathDescription: &'static str = "%Description";
const PathGlobalPercentage: &'static str = "%GlobalPercentage";
const PathIcon: &'static str = "%Icon";
const PathLabel: &'static str = "%Label";
const PathUnlockTime: &'static str = "%UnlockTime";

#[derive(GodotClass)]
#[class(base=HBoxContainer)]
pub struct Achievement
{
	#[base]
	base: Base<HBoxContainer>,
	
	#[export]
	pub description: GString,
	
	#[export]
	pub iconPath: GString,
	
	#[export]
	pub label: GString,
	
	#[export]
	pub globalPercentage: f64,
	
	#[export]
	pub unlockTime: GString,
}

impl Achievement
{
	pub fn updateData(&mut self, description: GString, iconPath: GString, label: GString, globalPercentage: f64, unlockTime: GString)
	{
		self.description = description;
		self.iconPath = iconPath;
		self.label = label;
		self.globalPercentage = globalPercentage;
		self.unlockTime = unlockTime;
		
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
		self.setLabelText(PathGlobalPercentage, format!("{}%", self.globalPercentage).into());
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
impl IHBoxContainer for Achievement
{
	fn init(base: Base<HBoxContainer>) -> Self
	{
		return Self
		{
			base,
			description: GString::default(),
			iconPath: GString::default(),
			label: GString::default(),
			globalPercentage: -1.0,
			unlockTime: GString::default(),
		};
	}
	
	fn ready(&mut self)
	{
		self.refresh();
	}
}
