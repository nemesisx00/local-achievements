use freya::hooks::{use_animation, AnimNum, Ease, Function, OnFinish};
use freya::prelude::{component, dioxus_elements, rsx, spawn, use_memo,
	use_signal, Element, GlobalSignal, Props, Readable, Writable};
use crate::constants::{BorderColor, CornerRadius};
use crate::{NotificationList, Settings};

const DefaultAnimationDuration: u64 = 500;

#[component]
pub fn NotificationElement(animationDuration: Option<u64>, displayDuration: Option<u64>) -> Element
{
	let duration = match animationDuration
	{
		None => DefaultAnimationDuration,
		Some(d) => d,
	};
	
	let hideDelay = match displayDuration
	{
		None => Settings().notificationDuration,
		Some(d) => d,
	};
	
	let mut text = use_signal(|| String::default());
	let mut notificationState = use_signal(|| NotificationState::default());
	
	let animation = use_animation(move |config| {
		//config.on_creation(OnCreation::Nothing);
		//config.on_finish(OnFinish::Nothing);
		config.on_finish(OnFinish::Stop);
		
		AnimNum::new(-250., 25.)
			.ease(Ease::InOut)
			.function(Function::Elastic)
			.time(duration)
	});
	
	let x = &*animation.get().read_unchecked();
	
	use_memo(move || {
		match notificationState()
		{
			NotificationState::Hidden => {
				if !NotificationList().is_empty()
				{
					if let Some(t) = NotificationList.write().pop_front()
					{
						text.set(t);
						animation.start();
						notificationState.set(NotificationState::Showing);
					}
				}
			},
			
			NotificationState::Showing => delay(duration, move || notificationState.set(NotificationState::Shown)),
			
			NotificationState::Shown => {
				delay(hideDelay, move || {
					notificationState.set(NotificationState::ShouldHide);
				});
			},
			
			NotificationState::ShouldHide => {
				animation.reverse();
				delay(duration, move || {
					notificationState.set(NotificationState::Hidden);
				});
			},
		}
	});
	
	let width = 100 + (text().len() * 2);
	
	return rsx!(
		rect
		{
			border: "1 center {BorderColor}",
			corner_radius: "{CornerRadius}",
			cross_align: "center",
			main_align: "center",
			margin: "5 10",
			min_height: "25",
			width: "{width}",
			position: "absolute",
			position_right: "{x.read()}",
			position_top: "5",
			
			label
			{
				main_align: "center",
				text_align: "center",
				
				"{text()}"
			}
		}
	);
}

fn delay<F>(delay: u64, closure: F)
	where F: FnOnce() -> () + 'static
{
	spawn({
		async move {
			tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
			closure();
		}
	});
}

#[derive(Clone, Copy, Debug, Default)]
enum NotificationState
{
	#[default]
	Hidden,
	ShouldHide,
	Showing,
	Shown,
}
