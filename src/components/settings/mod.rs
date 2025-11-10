mod local;
mod notifications;
mod settings;

use freya::prelude::{InputMode, Signal, Writable};

pub use settings::AppSettings;

pub const InputModeHiddenChar: char = '*';

pub fn toggleInputMode(signal: &mut Signal<InputMode>)
{
	match signal()
	{
		InputMode::Shown => signal.set(InputMode::Hidden(InputModeHiddenChar)),
		InputMode::Hidden(_) => signal.set(InputMode::Shown),
	}
}
