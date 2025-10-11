use freya::prelude::{component, dioxus_elements, fc_to_builder, rsx, spawn,
	use_hook, Element, GlobalSignal, IntoDynNode, Readable};
use crate::components::retroachievements::game::GameElement;
use crate::components::retroachievements::list::GameList;
use crate::{RetroAchievementsAuthData, RetroAchievementsUserData, SelectedGameId};
use crate::io::{saveUserData_RetroAchievements};
use crate::platforms::retroachievements::RetroAchievementsApi;

#[component]
pub fn RetroAchivementsContent() -> Element
{
	use_hook(|| if RetroAchievementsUserData().username.is_empty()
	{
		refresh();
	});
	
	let selectedGame = match SelectedGameId()
	{
		None => None,
		Some(gameId) => RetroAchievementsUserData().games.iter()
			.find(|g| g.id == gameId)
			.cloned(),
	};
	
	return rsx!(
		rect
		{
			cross_align: "center",
			direction: "vertical",
			spacing: "10",
			width: "fill",
			
			match selectedGame
			{
				None => rsx!(GameList {}),
				Some(game) => rsx!(GameElement { gameId: game.id }),
			}
		}
	);
}

pub fn refresh()
{
	spawn(async move {
		let api: RetroAchievementsApi = RetroAchievementsAuthData().into();
		loadUserProfile(&api).await;
		
		let mut progressState = loadUserCompletionProgress(&api, Default::default()).await;
		while !progressState.reachedEnd()
		{
			progressState = loadUserCompletionProgress(&api, progressState).await;
		}
		println!("Finished looping loadUserGameProgress");
		
		match saveUserData_RetroAchievements(&RetroAchievementsUserData())
		{
			Err(e) => println!("Error saving user data (RetroAchievements): {:}", e),
			Ok(_) => println!("Saved user data (RetroAchievements)"),
		}
	});
}

#[derive(Clone, Copy, Debug, Default)]
struct UserCompletionProgressState
{
	offset: usize,
	received: usize,
	total: usize,
}

impl UserCompletionProgressState
{
	pub fn reachedEnd(&self) -> bool
	{
		return self.received <= 0
			|| self.received >= self.total
			|| self.received % RetroAchievementsApi::GetUserGameCompletion_Count != 0;
	}
}

async fn loadUserCompletionProgress(api: &RetroAchievementsApi, state: UserCompletionProgressState) -> UserCompletionProgressState
{
	let mut newState = state.to_owned();
	
	match api.getUserCompletionProgress(
		RetroAchievementsUserData().ulid,
		Some(state.offset)
	).await
	{
		Err(e) => {
			println!("Error retrieving user game progress: {:?}", e);
			newState = UserCompletionProgressState::default();
		},
		
		Ok(payload) => {
			newState.received = state.received + payload.Count;
			newState.offset = state.offset + payload.Count;
			newState.total = payload.Total;
			
			RetroAchievementsUserData.write().processUserCompletionProgress(&payload);
			
			match api.cacheIcon_Games(&payload, false).await
			{
				Err(e) => println!("Error caching game icons: {:?}", e),
				Ok(_) => println!("Finished caching game icons"),
			}
		},
	}
	
	return newState;
}

async fn loadUserProfile(api: &RetroAchievementsApi)
{
	if let Ok(payload) = api.getUserProfile(RetroAchievementsUserData().ulid).await
	{
		RetroAchievementsUserData.write().processUserProfile(&payload);
		
		if let Some(ulid) = RetroAchievementsUserData().ulid
		{
			if let Some(avatarPath) = RetroAchievementsUserData().avatar
			{
				match api.cacheProfileAvatar(&ulid, &avatarPath, false).await
				{
					Err(e) => println!("Error caching avatar: {:?}", e),
					Ok(_) => println!("Avatar cached"),
				}
			}
		}
	}
}
