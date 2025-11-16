use freya::hooks::{cow_borrowed, theme_with, ButtonThemeWith};
use freya::prelude::{Button, Dropdown, DropdownItem, Element, GlobalSignal,
	IntoDynNode, Readable, Writable, component, dioxus_elements, fc_to_builder,
	rsx, use_memo, use_signal};
use strum::IntoEnumIterator;
use crate::{ActiveContent, GameSelected};

#[component]
pub fn NavBar() -> Element
{
	let mut selected = use_signal(|| ActiveContent());
	
	use_memo(move || {
		*ActiveContent.write() = selected();
	});
	
	return rsx!(
		rect
		{
			direction: "horizontal",
			width: "fill",
			
			rect
			{
				direction: "horizontal",
				main_align: "center",
				margin: "10",
				spacing: "5",
				width: "fill",
				
				if GameSelected()
				{
					Button
					{
						onpress: move |_| *GameSelected.write() = false,
						label { "Back" }
					}
				}
				
				Dropdown
				{
					value: selected(),
					
					for content in ActiveContent::iter()
						.filter(|ac| ac != &ActiveContent::Settings)
					{
						DropdownItem
						{
							value: content,
							onpress: move |_| {
								selected.set(content);
								*GameSelected.write() = false;
							},
							label { "{content}" }
						}
					}
				}
				
				Button
				{
					onclick: move |_| match ActiveContent()
					{
						ActiveContent::Settings => *ActiveContent.write() = selected(),
						_ => *ActiveContent.write() = ActiveContent::Settings,
					},
					theme: theme_with!(ButtonTheme {
						width: cow_borrowed!("100"),
					}),
					
					match ActiveContent()
					{
						ActiveContent::Settings => rsx!(label { text_align: "center", "Back" }),
						_ => rsx!(label { text_align: "center", "Settings" }),
					}
				}
			}
		}
	);
}
