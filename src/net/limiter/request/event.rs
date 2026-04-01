
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RequestEvent
{
	Added,
	Done,
	Processing(usize),
}
