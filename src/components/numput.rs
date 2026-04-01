use freya::prelude::{Component, Input, InputValidator, IntoElement, Size, State,
	use_side_effect, use_state};

#[derive(Clone, PartialEq)]
pub struct NumericInput
{
	value: State<u64>,
	max: u64,
	min: u64,
	placeholder: String,
	width: Size,
}

impl Component for NumericInput
{
	fn render(&self) -> impl IntoElement
	{
		let mut value = self.value.clone();
		let textValue = use_state(|| value.read().to_string());
		use_side_effect(move || value.set(textValue.read().parse::<u64>().unwrap()));
		
		let max = self.max;
		let min = self.min;
		
		return Input::new(textValue)
			.placeholder(self.placeholder.clone())
			.width(self.width.clone())
			.on_validate(move |validator| validate(validator, max, min));
	}
}

#[allow(unused)]
impl NumericInput
{
	pub fn new(value: State<u64>) -> Self
	{
		return Self
		{
			value,
			max: u64::MAX,
			min: u64::MIN,
			placeholder: String::default(),
			width: Size::default(),
		};
	}
	
	pub fn max(mut self, max: impl Into<u64>) -> Self
	{
		self.max = max.into();
		return self;
	}
	
	pub fn min(mut self, min: impl Into<u64>) -> Self
	{
		self.min = min.into();
		return self;
	}
	
	pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self
	{
		self.placeholder = placeholder.into();
		return self;
	}
	
	pub fn width(mut self, width: impl Into<Size>) -> Self
	{
		self.width = width.into();
		return self;
	}
}

fn validate(validator: InputValidator, max: u64, min: u64)
{
	validator.set_valid(match validator.text().parse::<u64>()
	{
		Err(_) => false,
		Ok(num) => min <= num && num <= max,
	});
}
