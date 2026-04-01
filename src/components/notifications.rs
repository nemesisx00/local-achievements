use std::cell::Cell;
use std::thread::sleep;
use std::time::Duration;
use freya::animation::{AnimNum, Ease, Function, OnCreation, OnFinish,
	use_animation};
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Direction, Gaps, IntoElement, Position, Size,
	StyleExt, TextAlign, TextStyleExt, label, rect, spawn, use_drop, use_hook,
	use_memo, use_state};
use freya::radio::use_radio;
use crate::constants::BorderColor;
use crate::data::AppData;
use crate::data::radio::AppDataChannel;

const DefaultAnimationDuration: u64 = 500;

#[derive(Clone, PartialEq)]
pub struct NotificationElement
{
	readyToRemove: Cell<bool>,
	text: String,
}

impl Component for NotificationElement
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Settings);
		
		let hideDelay = appData.read().app.settings.notificationDuration;
		
		let mut animation = use_animation(move |config| {
			config.on_creation(OnCreation::Nothing);
			config.on_finish(OnFinish::Nothing);
			
			AnimNum::new(-250.0, 25.0)
				.ease(Ease::InOut)
				.function(Function::Elastic)
				.time(DefaultAnimationDuration)
		});
		
		let mut hookAnim = animation.clone();
		use_hook(|| hookAnim.start());
		
		let mut dropAnim = animation.clone();
		use_drop(move || dropAnim.reverse());
		
		let mut readyToRemove = self.readyToRemove.clone();
		let mut notificationState = use_state(|| NotificationState::Hidden);
		use_memo(move || {
			let mut notificationState = notificationState.write();
			match notificationState.clone()
			{
				NotificationState::Hidden => {
					animation.start();
					*notificationState = NotificationState::Showing;
				},
				
				NotificationState::Showing => delay(DefaultAnimationDuration, move || *notificationState = NotificationState::Shown),
				
				NotificationState::Shown => delay(hideDelay, move || *notificationState = NotificationState::ShouldHide),
				
				NotificationState::ShouldHide => {
					animation.reverse();
					delay(DefaultAnimationDuration, move || *notificationState = NotificationState::ShouldRemove)
				},
				
				NotificationState::ShouldRemove => *readyToRemove.get_mut() = true,
			}
		});
		
		let text = self.text.clone();
		let x = animation.get().value();
		let width = 100 + (text.len() * 2);
		
		return rect()
			.border(Some(
				Border::new()
					.alignment(BorderAlignment::Center)
					.fill(BorderColor)
					.width(BorderWidth::from(1.0))
			))
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.main_align(Alignment::Center)
			.margin(Gaps::new_symmetric(5.0, 10.0))
			.min_height(Size::px(25.0))
			.position(Position::new_absolute()
				.right(x)
				.top(5.0)
			)
			.width(Size::px(width as f32))
			
			.child(
				label()
					.text_align(TextAlign::Center)
					.text(text)
			);
	}
}

impl NotificationElement
{
	pub fn new(text: String) -> Self
	{
		return Self
		{
			readyToRemove: Cell::new(false),
			text,
		};
	}
}

fn delay<F>(delay: u64, closure: F)
	where F: FnOnce() -> () + 'static
{
	println!("Spawning delay for {} milliseconds", delay);
	spawn({
		async move {
			println!("Waiting {} milliseconds", delay);
			sleep(Duration::from_millis(delay));
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
	ShouldRemove,
	Showing,
	Shown,
}
