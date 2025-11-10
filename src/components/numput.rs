use freya::prelude::{component, fc_to_builder, rsx, Element, GlobalSignal, Input,
	InputValidator, Props, Readable, Signal, Writable};

#[component]
pub fn NumericInput(
	value: Signal<u64>,
	max: Option<u64>,
	min: Option<u64>,
	placeholder: Option<String>,
	width: Option<String>
) -> Element
{
	let max = match max
	{
		None => u64::MAX,
		Some(m) => m,
	};
	
	let min = match min
	{
		None => u64::MIN,
		Some(m) => m,
	};
	
	let placeholder = match placeholder
	{
		None => "500".into(),
		Some(p) => p,
	};
	
	let width = match width
	{
		None => "auto".into(),
		Some(w) => w,
	};
	
	return rsx!(
		Input
		{
			placeholder,
			value: value().to_string(),
			width,
			onchange: move |text: String| value.set(text.parse::<u64>().unwrap()),
			onvalidate: move |validator: InputValidator| validate(validator, max, min),
		}
	);
}

fn validate(validator: InputValidator, max: u64, min: u64)
{
	validator.set_valid(match validator.text().parse::<u64>()
	{
		Err(_) => false,
		Ok(num) => min <= num && num <= max,
	});
}
