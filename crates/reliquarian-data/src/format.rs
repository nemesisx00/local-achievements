pub fn truncateF32(value: impl Into<f32>, decimalPlaces: impl Into<i32>) -> f32
{
	let pow = 10u32.pow(decimalPlaces.into() as u32) as f32;
	return (value.into() * pow).round() / pow;
}

#[allow(unused)]
pub fn truncateF64(value: impl Into<f64>, decimalPlaces: impl Into<i32>) -> f64
{
	let pow = 10u32.pow(decimalPlaces.into() as u32) as f64;
	return (value.into() * pow).round() / pow;
}
