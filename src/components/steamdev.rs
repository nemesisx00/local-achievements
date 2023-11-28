#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::dioxus::prelude::*;
use crate::{transmit, userData};
use crate::background::{ApiCommand, Internal, SteamEndpoint};

/**
Buttons for use during development. Will be phased out as the application
is filled out.
*/
pub fn SteamDev(cx: Scope) -> Element
{
	return cx.render(rsx!
	{
		div
		{
			class: "steamDev",
			
			div
			{
				button
				{
					onclick: move |_| transmit(ApiCommand::Print("This message was sent from the frontend!".into())),
					"ApiCommand::Print Test"
				}
			}
			
			div
			{
				button
				{
					onclick: move |_| {
						if let Ok(user) = userData().lock()
						{
							println!("Steam Info: {:?}", user.steam);
						}
					},
					"Print Steam Info"
				}
				
				button
				{
					onclick: move |_| transmit(ApiCommand::Metadata(Internal::SaveUserData)),
					"Save Data"
				}
				
				button
				{
					onclick: move |_| transmit(ApiCommand::Steam(SteamEndpoint::PlayerSummaries)),
					"Get Player Summaries"
				}
			}
			
			div
			{
				button
				{
					onclick: move |_| transmit(ApiCommand::Steam(SteamEndpoint::OwnedGames)),
					"Get Owned Games"
				}
				
				button
				{
					onclick: move |_| transmit(ApiCommand::Steam(SteamEndpoint::GlobalPercentages(389730))),
					"Get Global Percentages"
				}
				
				button
				{
					onclick: move |_| transmit(ApiCommand::Steam(SteamEndpoint::RecentlyPlayedGames)),
					"Get Recently Played Games"
				}
			}
		}
	});
}
