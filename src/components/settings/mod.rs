mod retroachievements;
mod settings;
mod steam;

use freya::prelude::{InputMode, Signal, Writable};

pub use settings::AppSettings;

const InputModeHiddenChar: char = '*';

fn toggleInputMode(signal: &mut Signal<InputMode>)
{
	match signal()
	{
		InputMode::Shown => signal.set(InputMode::Hidden(InputModeHiddenChar)),
		InputMode::Hidden(_) => signal.set(InputMode::Shown),
	}
}
