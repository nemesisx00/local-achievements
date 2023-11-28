/*!
Local Achievements is an open source desktop application for collecting, storing,
and tracking your achievements across multiple platforms in one unified UI.
*/

#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod background;
mod components;
mod data;
mod hooks;
mod io;
mod macros;
mod platforms;
mod state;

use std::sync::{OnceLock, Mutex};
use background::CommandResponse;
use ::dioxus_desktop::{Config, LogicalSize, WindowBuilder, launch_cfg};
use tokio::sync::mpsc::UnboundedReceiver;
use ::tokio::sync::mpsc::{self, UnboundedSender};
use crate::background::{ApiCommand, Dispatcher};
use crate::components::App;
use crate::data::User;
use crate::state::loadUserData;

pub const AppTitle: &str = "Local Achievements";
pub const AppVersion: &str = "0.2.0";
pub const BackgroundColor: (u8, u8, u8, u8) = (26, 26, 26, 255);
pub const CssFile: &str = "app.css";
pub const MinimumWindowSize: LogicalSize<u32> = LogicalSize::new(720, 480);
pub const DefaultWindowSize: LogicalSize<u32> = LogicalSize::new(1280, 720);

#[tokio::main]
async fn main()
{
	let win = WindowBuilder::default()
		.with_inner_size(DefaultWindowSize)
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
	
	//Set up the communication channels
	let (frontendSender, backendReceiver) = mpsc::unbounded_channel::<ApiCommand>();
	*transmitter().lock().unwrap() = Some(frontendSender);
	let (backendSender, frontendReceiver) = mpsc::unbounded_channel::<CommandResponse>();
	*receiver().lock().unwrap() = Some(frontendReceiver);
	
	tokio::spawn(async move {
		let mut dispatcher = Dispatcher::new(backendReceiver, backendSender);
		dispatcher.start().await;
	});
	
	launch_cfg(App, config);
}

/**
Retrieve a static reference to the sender used to transmit commands to the
background task.
*/
fn transmitter() -> &'static Mutex<Option<UnboundedSender<ApiCommand>>>
{
	static Channel: OnceLock<Mutex<Option<UnboundedSender<ApiCommand>>>> = OnceLock::new();
	return Channel.get_or_init(|| Mutex::new(None));
}

fn receiver() -> &'static Mutex<Option<UnboundedReceiver<CommandResponse>>>
{
	static Channel: OnceLock<Mutex<Option<UnboundedReceiver<CommandResponse>>>> = OnceLock::new();
	return Channel.get_or_init(|| Mutex::new(None));
}

/**
Transmit a single command to the background task.
*/
pub fn transmit(command: ApiCommand)
{
	if let Ok(channel) = transmitter().lock()
	{
		if let Some(tx) = channel.as_ref()
		{
			let _ = tx.send(command);
		}
	}
}

/**
Transmit a list of commands to the background task.
*/
pub fn transmitMultiple(commands: Vec<ApiCommand>)
{
	if let Ok(channel) = transmitter().lock()
	{
		if let Some(tx) = channel.as_ref()
		{
			for command in commands
			{
				let _ = tx.send(command);
			}
		}
	}
}

/**
Retrieve a static reference to the user data.
*/
pub fn userData() -> &'static Mutex<User>
{
	static Data: OnceLock<Mutex<User>> = OnceLock::new();
	return Data.get_or_init(|| Mutex::new(match loadUserData()
	{
		Ok(user) => user,
		Err(_) => User::default()
	}));
}
