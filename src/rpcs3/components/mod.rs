pub mod content;
pub mod game;
pub mod list;
pub mod profile;
pub mod settings;
pub mod trophy;

use freya::prelude::{GlobalSignal, Signal};

static SelectedGameId: GlobalSignal<Option<String>> = Signal::global(|| None);
