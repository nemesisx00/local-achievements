use data::constants::InputModeHiddenChar;
use freya::prelude::{ChildrenExt, ContainerExt, ContainerSizeExt, Gaps,
	InputMode, IntoElement, Size, Switch, rect};
use freya::radio::Writable;

pub fn SettingsSwitch(mut inputMode: Writable<InputMode>) -> impl IntoElement
{
	let value = inputMode.read().clone();
	
	return rect()
		.margin(Gaps::new(4.0, 0.0, 0.0, 0.0))
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
