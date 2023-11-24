#![allow(non_snake_case, non_upper_case_globals)]

use std::path::Path;
use ::lightningcss::bundler::{Bundler, FileProvider};
use ::lightningcss::stylesheet::{ParserOptions, PrinterOptions};

const CssFile: &str = "./css/app.css";
const OutputPath: &str = "./static/app.css";

fn main()
{
	let css = css();
	let _ = std::fs::write(OutputPath, css.clone());
	
	//Only re-run if the CSS has actually changed
	println!("cargo:rerun-if-changed=css/**/*.css");
}

pub fn css() -> String
{
	let path = Path::new(CssFile);
	let fs = FileProvider::new();
	let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
	let printerOptions = PrinterOptions { minify: true, ..Default::default() };
	let css = match bundler.bundle(path).as_mut()
	{
		Ok(stylesheet) => match stylesheet.to_css(printerOptions)
		{
			Ok(css) => css.code,
			Err(_) => String::default(),
		},
		Err(_) => String::default(),
	};
	
	return css;
}
