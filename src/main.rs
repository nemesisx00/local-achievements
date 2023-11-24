/*!
Local Achievements is an open source desktop application for collecting, storing,
and tracking your achievements across multiple platforms in one unified UI.
*/

#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod components;
mod data;
mod io;
mod macros;
mod platforms;
mod state;

use ::dioxus_desktop::{Config, LogicalSize, WindowBuilder, launch_cfg};
use crate::components::App;

pub const AppTitle: &str = "Local Achievements";
pub const AppVersion: &str = "0.2.0";
pub const BackgroundColor: (u8, u8, u8, u8) = (26, 26, 26, 255);
pub const CssFile: &str = "app.css";
pub const MinimumWindowSize: LogicalSize<u32> = LogicalSize::new(720, 480);

fn main()
{
	let win = WindowBuilder::default()
		.with_inner_size(MinimumWindowSize)
		.with_min_inner_size(MinimumWindowSize)
		.with_title(format!("{} {}", AppTitle, AppVersion));
	
	//TODO: Find a better solution for having working styles in dx serve
	let css = format!(r#"<link rel="stylesheet" href="{}" />"#, match cfg!(debug_assertions)
	{
		true =>  format!("static/{}", CssFile),
		false => CssFile.to_string(),
	});
	
	let config = Config::default()
		.with_background_color(BackgroundColor)
		.with_custom_head(css)
		.with_window(win);
	
	launch_cfg(App, config);
}
