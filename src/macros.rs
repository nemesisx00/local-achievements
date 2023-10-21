#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

/**
Generate an anyhow::Error from a std::io::Error, given a std::io::ErrorKind.
*/
#[macro_export]
macro_rules! error
{
	($kind:expr) => {
		{
			use ::anyhow::Error;
			Error::from(std::io::Error::from($kind))
		}
	}
}
