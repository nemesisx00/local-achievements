use freya::hooks::{AnimNum, Ease, Function, OnFinish};
use freya::prelude::{Button, Element, GlobalSignal, Props, Readable, Writable,
	component, dioxus_elements, fc_to_builder, rsx, use_animation, use_signal};
use crate::constants::{ButtonBackgroundColor, BorderColor};
use crate::retroachievements::RetroAchievementsUserProfile;
use crate::steam::SteamProfile;

#[component]
pub fn ProfileElement(duration: Option<u64>, ) -> Element
{
	let duration = match duration
	{
		None => 500,
		Some(d) => d,
	};
	let width = 250.;
	let paddedWidth = width + 15.;
	let paddedEnd = -15.;
	
	let mut visible = use_signal(|| false);
	
	let animation = use_animation(move |config| {
		//config.on_creation(OnCreation::Nothing);
		//config.on_finish(OnFinish::Nothing);
		config.auto_start(false);
		config.on_finish(OnFinish::Stop);
		
		AnimNum::new(-paddedWidth, paddedEnd)
			.ease(Ease::InOut)
			.function(Function::Cubic)
			.time(duration)
	});
	
	let x = &*animation.get().read_unchecked();
	
	if x.read() == -paddedWidth as f32
	{
		visible.set(false);
	}
	else if x.read() == paddedEnd
	{
		visible.set(true);
	}
	
	let buttonLabel = match visible()
	{
		false => ">",
		true => "<",
	};
	
	return rsx!(
		rect
		{
			background: "{ButtonBackgroundColor}",
			border: "none, 1 center {BorderColor}, none, none",
			content: "flex",
			direction: "vertical",
			height: "100v",
			margin: "0",
			layer: "-1",
			padding: "15",
			position: "absolute",
			position_left: "{x.read()}",
			position_top: "-15",
			spacing: "10",
			width: "{width}",
			
			rect
			{
				layer: "-1",
				position: "absolute",
				position_right: "-15",
				position_top: "0",
				
				Button
				{
					onpress: move |_| {
						if visible()
						{
							animation.reverse();
						}
						else
						{
							animation.start();
						}
					},
					
					label
					{
						font_size: "40",
						font_weight: "bold",
						
						"{buttonLabel}"
					}
				}
			}
			
			label
			{
				text_align: "center",
				width: "flex",
				"RetroAchievements"
			}
			RetroAchievementsUserProfile {}
			
			rect
			{
				border: "none, none, 1 center {BorderColor}",
				margin: "7 0",
				width: "flex",
			}
			
			label
			{
				text_align: "center",
				width: "flex",
				"Steam"
			}
			SteamProfile {}
		}
	);
}
