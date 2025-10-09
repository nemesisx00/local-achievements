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

/**
Quickly turn a string or string slice into a JPEG filename.
*/
#[macro_export]
macro_rules! jpg {
	($name:expr) => {
		format!("{}.jpg", $name)
	};
}

#[macro_export]
macro_rules! jpgAlt {
	($name:expr, $alt:expr) => {
		format!("{}_{}.jpg", $name, $alt)
	};
}

/**
Quickly turn a string or string slice into a PNG filename.
*/
#[macro_export]
macro_rules! png {
	($name:expr) => {
		format!("{}.png", $name)
	};
}

#[macro_export]
macro_rules! pngAlt {
	($name:expr, $alt:expr) => {
		format!("{}_{}.png", $name, $alt)
	};
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
	
	#[test]
	fn jpg()
	{
		let name = "image";
		let expected = "image.jpg".to_string();
		let result1 = jpg!(name);
		let result2 = jpg!(name.to_string());
		
		assert_eq!(expected, result1);
		assert_eq!(expected, result2);
	}
	
	#[test]
	fn jpgAlt()
	{
		let name = "image";
		let alt = "other";
		let expected = "image_other.jpg".to_string();
		let result1 = jpgAlt!(name, alt);
		let result2 = jpgAlt!(name.to_string(), alt.to_string());
		
		assert_eq!(expected, result1);
		assert_eq!(expected, result2);
	}
	
	#[test]
	fn png()
	{
		let name = "image";
		let expected = "image.png".to_string();
		let result1 = png!(name);
		let result2 = png!(name.to_string());
		
		assert_eq!(expected, result1);
		assert_eq!(expected, result2);
	}
	
	#[test]
	fn pngAlt()
	{
		let name = "image";
		let alt = "other";
		let expected = "image_other.png".to_string();
		let result1 = pngAlt!(name, alt);
		let result2 = pngAlt!(name.to_string(), alt.to_string());
		
		assert_eq!(expected, result1);
		assert_eq!(expected, result2);
	}
}
