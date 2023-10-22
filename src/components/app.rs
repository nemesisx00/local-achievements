#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use ::fermi::use_atom_ref;
use crate::io::writeAuth_Steam;
use crate::platforms::retroachievements::AuthObject;
use crate::platforms::steam::{AuthData, SteamApi};
use crate::state::{User, loadState, saveState};

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
	fermi::use_init_atom_root(cx);
	
	let user = use_atom_ref(cx, &User);
	
	let api = use_ref(cx, || match cx.props.steamAuth.is_some() { true => SteamApi::new(cx.props.steamAuth.as_ref().unwrap().clone()).unwrap(), false => SteamApi::default() });
	let id = use_state(cx, || match cx.props.steamAuth.is_some() { true => cx.props.steamAuth.as_ref().unwrap().id.to_owned(), false => String::new() });
	let apiKey = use_state(cx, || match cx.props.steamAuth.is_some() { true => cx.props.steamAuth.as_ref().unwrap().key.to_owned(), false => String::new() });
	
	let apiClone = api.read().clone();
	let future = use_future(cx, (), |_| async move
	{
		return apiClone.getPlayerSummaries().await;
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
					let _result = writeAuth_Steam(auth.clone());
					api.write().auth = auth.to_owned();
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
					if api.read().auth.validate()
					{
						future.restart();
						match future.value()
						{
							Some(result) => match result
							{
								Ok(summary) => {
									println!("{:?}", summary);
									
									match summary.response.players.first()
									{
										Some(profile) => {
											let mut userRef = user.write();
											userRef.steam.name = profile.personaname.clone();
											userRef.steam.id = profile.steamid.clone();
										},
										None => {},
									}
									
								},
								Err(_) => println!("Failed to complete request!"),
							},
							None => println!("No future value!"),
						}
					}
				},
				"Run GetPlayerSummaries Request"
			}
			button
			{
				onclick: move |_| println!("Steam Info: {:?}", user.read().steam),
				"Print Steam Info"
			}
			
			button
			{
				onclick: move |_| {
					match loadState(cx)
					{
						Ok(_) => println!("Profile loaded!"),
						Err(_) => println!("Failed to load profile!"),
					}
				},
				"Load Profile Data"
			}
			
			button
			{
				onclick: move |_| {
					match saveState(cx)
					{
						Ok(_) => println!("Profile saved!"),
						Err(_) => println!("Failed to save profile!"),
					}
				},
				"Save Profile Data"
			}
		}
	});
}
