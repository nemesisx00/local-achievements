pub mod achievement;
pub mod content;
pub mod game;
pub mod list;
pub mod profile;
pub mod settings;

use freya::prelude::{GlobalSignal, Signal};

static SelectedGameId: GlobalSignal<Option<usize>> = Signal::global(|| None);
