/*!
The data module contains all data structures used by the local-achievements
application.

The data structres are designed specifically for local-achievements and are the
final form into which the data from the platforms module is parsed.
*/

mod data;
pub mod radio;
pub mod secure;
mod settings;
mod state;

pub use data::AppData;

pub use settings::{
	AppSettings,
	//DefaultNotificationDuration
};

pub use state::{
	active::ActiveContent,
	app::AppState,
	platform::{GamePlatforms, PlatformState},
	user::UserState,
};
