pub mod achievement;
pub mod game;
pub mod kind;
pub mod mode;
pub mod rank;
pub mod system;
pub mod user;

/**
Make an absolute path relative by removing the leading '/' if it exists.
*/
pub fn makeRelative(value: &String) -> String
{
	let mut path = value.to_owned();
	return match path.starts_with("/")
	{
		false => path,
		true => path.split_off(1),
	};
}
