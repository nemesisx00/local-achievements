use data::constants::BorderColor;
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Direction, Gaps, IntoElement, Size, StyleExt, rect};

pub fn separatorElement() -> impl IntoElement
{
	return rect()
		.cross_align(Alignment::Center)
		.direction(Direction::Horizontal)
		.height(Size::px(1.0))
		.main_align(Alignment::Center)
		.margin(Gaps::new(0.0, 0.0, 15.0, 0.0))
		.width(Size::Fill)
		
		.child(
			rect()
				.border(Some(
					Border::new()
						.alignment(BorderAlignment::Center)
						.fill(BorderColor)
						.width(BorderWidth
						{
							top: 0.0,
							right: 0.0,
							bottom: 1.0,
							left: 0.0,
						})
				))
				.height(Size::px(1.0))
				.width(Size::percent(40.0))
		);
}
