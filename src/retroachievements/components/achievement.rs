use freya::hooks::cow_borrowed;
use freya::prelude::{component, dioxus_elements, dynamic_bytes, fc_to_builder,
	rsx, theme_with, Element, GlobalSignal, IntoDynNode, Loader, ProgressBar,
	ProgressBarThemeWith, Props, Readable};
use crate::io::{loadImageToBytes, Path_Games};
use crate::constants::{BorderColor, Icon_Locked,
	RetroAchievementsProgressColorBackground, RetroAchievementsProgressColorCasual,
	RetroAchievementsProgressColorHardcore};
use crate::retroachievements::RetroAchievementsMode;
use crate::retroachievements::platform::api::Api;
use crate::{join, png, pngAlt, RetroAchievementsUserData};

#[component]
pub fn AchievementElement(gameId: usize, achievementId: usize) -> Element
{
	let game = match RetroAchievementsUserData().games.iter()
		.find(|g| g.id == gameId)
	{
		None => Default::default(),
		Some(g) => g.to_owned(),
	};
	
	let achievement = match game.achievements.iter()
		.find(|a| a.id == achievementId)
	{
		None => Default::default(),
		Some(achievement) => achievement.to_owned(),
	};
	
	let unlockedCasual = achievement.unlocked(RetroAchievementsMode::Casual);
	let unlockedHardcore = achievement.unlocked(RetroAchievementsMode::Hardcore);
	
	let iconName = match unlockedCasual || unlockedHardcore
	{
		true => png!(achievement.name),
		false => pngAlt!(achievement.name, Icon_Locked),
	};
	
	let bytes = loadIcon(
		&Api::Platform.to_lowercase(),
		&join!(Path_Games, gameId),
		&iconName
	);
	
	let timestamp = match achievement.formatEarnedTimestamp(match unlockedHardcore {
			false => RetroAchievementsMode::Casual,
			true => RetroAchievementsMode::Hardcore,
		})
	{
		Err(_) => String::default(),
		Ok(ts) => ts,
	};
	
	let percentCasual = achievement.unlockedPercent(
		RetroAchievementsMode::Casual,
		game.distinctPlayers
	);
	
	let percentHardcore = achievement.unlockedPercent(
		RetroAchievementsMode::Hardcore,
		game.distinctPlayers
	);
	
	let percentCasualString = format!("{:.2}", percentCasual);
	let percentHardcoreString = format!("{:.2}", percentHardcore);
	
	return rsx!(
		rect
		{
			direction: "horizontal",
			main_align: "space-around",
			margin: "5 0",
			width: "fill",
			
			rect
			{
				border: "1 center {BorderColor}",
				corner_radius: "10",
				direction: "horizontal",
				main_align: "space-between",
				margin: "5",
				min_height: "64",
				min_width: "540",
				padding: "10 15",
				spacing: "10",
				width: "50%",
				
				rect
				{
					direction: "horizontal",
					min_height: "64",
					spacing: "10",
					width: "fill",
					
					match bytes.is_empty()
					{
						true => rsx!(Loader{}),
						false => rsx!(image
						{
							image_data: dynamic_bytes(bytes),
							width: "64",
						})
					}
					
					rect
					{
						direction: "vertical",
						height: "100%",
						main_align: "space-between",
						spacing: "15",
						width: "60%",
						
						label { "{achievement.name}" }
						label { font_size: "10", max_height: "48", "{achievement.description}" }
					}
				}
				
				rect
				{
					cross_align: "end",
					direction: "vertical",
					main_align: "space-between",
					min_width: "150",
					height: "100%",
					width: "150",
					
					rect
					{
						cross_align: "end",
						direction: "vertical",
						main_align: "start",
						width: "fill",
						
						rect
						{
							layer: "2",
							position: "absolute",
							position_right: "0",
							position_top: "0",
							width: "150",
							
							ProgressBar
							{
								progress: percentCasual as f32,
								theme: theme_with!(ProgressBarTheme {
									background: cow_borrowed!(RetroAchievementsProgressColorBackground),
									height: cow_borrowed!("8"),
									progress_background: cow_borrowed!(RetroAchievementsProgressColorCasual),
								}),
							}
						}
						
						rect
						{
							layer: "1",
							position: "absolute",
							position_right: "0",
							position_top: "0",
							width: "150",
							
							ProgressBar
							{
								progress: percentHardcore as f32,
								theme: theme_with!(ProgressBarTheme {
									background: cow_borrowed!("transparent"),
									height: cow_borrowed!("8"),
									progress_background: cow_borrowed!(RetroAchievementsProgressColorHardcore),
								}),
							}
						}
						
						paragraph
						{
							margin: "10 0 0 0",
							text_align: "center",
							text { font_size: "10", "{achievement.awardedCasual} " }
							text { font_size: "10", font_weight: "bold", "({achievement.awardedHardcore})" }
							text { font_size: "10", " of {game.distinctPlayers}" }
						}
						
						paragraph
						{
							text_align: "center",
							text { font_size: "10", "{percentCasualString}% " }
							text { font_size: "10", font_weight: "bold", "({percentHardcoreString}%)" }
							text { font_size: "10", " unlock rate" }
						}
					}
					
					label { font_size: "10", text_align: "center", width: "fill", "{timestamp}" }
				}
			}
		}
	);
}

fn loadIcon<'a>(platform: &String, group: &String, fileName: &String) -> Vec<u8>
{
	return match loadImageToBytes(platform, group, fileName)
	{
		Ok(b) => b,
		Err(_) => vec![],
	};
}
