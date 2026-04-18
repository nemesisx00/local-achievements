use data::constants::InputModeHiddenChar;
use freya::prelude::{Alignment, ChildrenExt, ContainerSizeExt,
	ContainerWithContentExt, InputMode, IntoElement, Size, Switch, rect};
use freya::radio::Writable;

pub fn InputModeSwitch(mut inputMode: Writable<InputMode>) -> impl IntoElement
{
	let value = inputMode.read().clone();
	
	return rect()
		.cross_align(Alignment::Center)
		.main_align(Alignment::Center)
		.width(Size::FillMinimum)
		
		.child(
			Switch::new()
				.toggled(value == InputMode::Shown)
				.on_toggle(move |_| match value
				{
					InputMode::Shown => *inputMode.write() = InputMode::Hidden(InputModeHiddenChar),
					InputMode::Hidden(_) => *inputMode.write() = InputMode::Shown,
				})
		);
}
