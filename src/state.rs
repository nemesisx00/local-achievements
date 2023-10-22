#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::Path;
use ::anyhow::Result;
use dioxus::prelude::Properties;
use ::dioxus::prelude::Scope;
use ::fermi::{use_atom_ref, AtomRef};
use crate::data::User;
use crate::error;
use crate::io::getDataDir;

const Filename: &str = "data.json";

pub static User: AtomRef<User> = AtomRef(|_| User::default());

pub fn loadState<T>(cx: Scope<T>) -> Result<()>
	where T: Properties
{
	let user = use_atom_ref(cx, &User);
	
	if let Some(dir) = getDataDir(false)
	{
		let path = Path::new(dir.as_str()).join(Filename);
		let file = File::open(&path.as_path())?;
		let buffer = BufReader::new(file);
		let instance: User = serde_json::from_reader(buffer)?;
		*user.write() = instance;
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

pub fn saveState<T>(cx: Scope<T>) -> Result<()>
	where T: Properties
{
	let user = use_atom_ref(cx, &User);
	
	if let Some(dir) = getDataDir(true)
	{
		let path = Path::new(dir.as_str()).join(Filename);
		let file = File::create(&path.as_path())?;
		let buffer = BufWriter::new(file);
		serde_json::to_writer(buffer, &user.read().clone())?;
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}
