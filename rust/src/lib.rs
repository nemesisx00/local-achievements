#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::godot::init::{gdextension, ExtensionLibrary};

mod data;
mod nodes;
mod io;
mod macros;
mod platforms;

struct LocalAchievements;

#[gdextension]
unsafe impl ExtensionLibrary for LocalAchievements {}
