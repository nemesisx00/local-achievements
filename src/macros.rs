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

#[macro_export]
macro_rules! join
{
	($a:expr, $($val:expr),+) => {
		{
			let mut s = format!("{}", $a);
			$( s = format!("{}/{}", s, $val); )+
			s
		}
	}
}

#[macro_export]
macro_rules! joinSep
{
	($sep:expr, $a:expr, $($val:expr),+) => {
		{
			let mut s = format!("{}", $a);
			$( s = format!("{}{}{}", s, $sep, $val); )+
			s
		}
	}
}

#[cfg(test)]
mod tests
{
	#[test]
	fn join()
	{
		let a = "1";
		let b = "z";
		let c = "what";
		
		let r1 = join!(a, b);
		let r2 = join!(a, b, c);
		
		let e1 = String::from("1/z");
		let e2 = String::from("1/z/what");
		
		assert_eq!(e1, r1);
		assert_eq!(e2, r2);
	}
	
	#[test]
	fn joinSep()
	{
		let separator = "/";
		let a = "1";
		let b = "z";
		let c = "what";
		
		let r1 = joinSep!(separator, a, b);
		let r2 = joinSep!(separator, a, b, c);
		
		let e1 = String::from("1/z");
		let e2 = String::from("1/z/what");
		
		assert_eq!(e1, r1);
		assert_eq!(e2, r2);
	}
}
