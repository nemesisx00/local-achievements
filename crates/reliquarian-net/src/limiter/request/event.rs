use data::enums::DataChannel;
use freya::radio::RadioChannel;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RequestEvent
{
	Added,
	Done,
	Processing(usize),
}

impl RadioChannel<RequestEvent> for DataChannel {}
