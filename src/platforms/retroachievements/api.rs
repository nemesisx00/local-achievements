#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use super::data::AuthObject;

#[derive(Clone, Debug, Default)]
pub struct Api
{
	pub auth: AuthObject,
}
