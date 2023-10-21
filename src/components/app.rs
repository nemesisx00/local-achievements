#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use crate::io::writeAuth_Steam;
use crate::platforms::retroachievements::AuthObject;
use crate::platforms::steam::{AuthData, SteamApi};

#[derive(PartialEq, Props)]
pub struct AppProps
{
	pub retroAuth: Option<AuthObject>,
	pub steamAuth: Option<AuthData>,
}

/**
The root component of the application.
*/
pub fn App<'a>(cx: Scope<AppProps>) -> Element
{
	let api = use_ref(cx, || SteamApi::default());
	let id = use_state(cx, || match cx.props.steamAuth.is_some() { true => cx.props.steamAuth.as_ref().unwrap().id.to_owned(), _ => String::new() });
	let apiKey = use_state(cx, || match cx.props.steamAuth.is_some() { true => cx.props.steamAuth.as_ref().unwrap().key.to_owned(), _ => String::new() });
	let shouldRunFuture = use_state(cx, || true);
	
	let apiClone = api.read().clone();
	to_owned![shouldRunFuture];
	let future = use_future(cx, (), |_| async move
	{
		println!("{:?}", apiClone.auth.clone());
		let response = apiClone.getPlayerSummaries().await;
		let value = match response
		{
			Ok(summary) => format!("{:?}", summary),
			Err(e) => format!("{:?}", e),
		};
		
		shouldRunFuture.set(false);
		return value;
	});
	
	return cx.render(rsx!
	{
		h1 { "Local Achievements" }
		hr {}
		div
		{
			h3 { "Update Steam Auth Info" }
			div
			{
				label { r#for: "authId", "Steam ID:" }
				input { name: "authId", r#type: "text", value: "{id}", onchange: move |e| id.set(e.value.clone()) }
			}
			div
			{
				label { r#for: "authApiKey", "API Key:" }
				input { name: "authApiKey", r#type: "text", value: "{apiKey}", onchange: move |e| apiKey.set(e.value.clone()) }
			}
			button
			{
				onclick: move |_| {
					let auth = AuthData { id: id.to_string(), key: apiKey.to_string() };
					let _result = writeAuth_Steam(auth);
				},
				"Update"
			}
		}
		hr {}
		div
		{
			h3 { "Steam" }
			button
			{
				onclick: move |_|
				{
					if let Some(auth) = &cx.props.steamAuth
					{
						api.write().auth = auth.to_owned();
					}
					
					future.restart();
					
					if let Some(summary) = future.value()
					{
						println!("{}", summary);
					}
				},
				"Run GetPlayerSummaries Request"
			}
		}
	});
}
