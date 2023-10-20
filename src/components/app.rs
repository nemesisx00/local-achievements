#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use crate::data::retroachievements::AuthObject;
use crate::io::writeAuth_RetroAchievements;

#[derive(PartialEq, Props)]
pub struct AppProps
{
	pub auth: Option<AuthObject>,
}

/**
The root component of the application.
*/
pub fn App(cx: Scope<AppProps>) -> Element
{
	let username = use_state(cx, || match cx.props.auth.is_some() { true => cx.props.auth.as_ref().unwrap().username.to_owned(), _ => String::new() });
	let apiKey = use_state(cx, || match cx.props.auth.is_some() { true => cx.props.auth.as_ref().unwrap().key.to_owned(), _ => String::new() });
	
	return cx.render(rsx!
	{
		h1 { "Local Achievements" }
		hr {}
		div
		{
			h3 { "Update Retro Achievements Auth Info" }
			div
			{
				label { r#for: "authUsername", "Username:" }
				input { name: "authUsername", r#type: "text", value: "{username}", onchange: move |e| username.set(e.value.clone()) }
			}
			div
			{
				label { r#for: "authApiKey", "API Key:" }
				input { name: "authApiKey", r#type: "text", value: "{apiKey}", onchange: move |e| apiKey.set(e.value.clone()) }
			}
			button
			{
				onclick: move |_| {
					let auth = AuthObject { username: username.to_string(), key: apiKey.to_string() };
					let _result = writeAuth_RetroAchievements(auth);
					
					username.set(String::new());
					apiKey.set(String::new());
				},
				"Update"
			}
		}
		hr {}
		div
		{
			h3 { "Retro Achievements" }
		}
	});
}
