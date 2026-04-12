use freya::prelude::{CursorIcon, Event, EventHandler, EventHandlersExt, Label,
	Platform, PressEventData, Rect, WinitPlatformExt};
use freya::radio::Writable;

pub trait PressableExt
	where Self: EventHandlersExt,
{
	/**
	Handle press events and also set the cursor to `CursorIcon::Pointer` while
	the pointer is over the element.
	*/
	fn pressable(self, handler: impl Into<EventHandler<Event<PressEventData>>>) -> Self
	{
		let handler = handler.into();
		return self.on_pointer_enter(move |_| {
			Platform::get().with_window(
				None,
				move |window| window.set_cursor(CursorIcon::Pointer)
			);
		})
		
		.on_pointer_leave(move |_| {
			Platform::get().with_window(
				None,
				move |window| window.set_cursor(CursorIcon::default())
			);
		})
		
		.on_press(move |e| {
			Platform::get().with_window(
				None,
				move |window| window.set_cursor(CursorIcon::default())
			);
			handler.call(e);
		});
	}
	
	/**
	Handle press events and also set the cursor to `CursorIcon::Pointer` while
	the pointer is over the element.
	
	Also accepts a `Writable<bool>` to signal the state of the pointer on enter
	and leave.
	*/
	fn pressableWithHover(
		self,
		hovering: impl Into<Writable<bool>>,
		handler: impl Into<EventHandler<Event<PressEventData>>>
	) -> Self
	{
		let handler = handler.into();
		let mut hovering = hovering.into();
		let mut hoveringLeave = hovering.clone();
		
		return self.on_pointer_enter(move |_| {
			Platform::get().with_window(
				None,
				move |window| window.set_cursor(CursorIcon::Pointer)
			);
			*hovering.write() = true;
		})
		
		.on_pointer_leave(move |_| {
			Platform::get().with_window(
				None,
				move |window| window.set_cursor(CursorIcon::default())
			);
			*hoveringLeave.write() = false;
		})
		
		.on_press(move |e| {
			Platform::get().with_window(
				None,
				move |window| window.set_cursor(CursorIcon::default())
			);
			handler.call(e);
		});
	}
}

impl PressableExt for Label {}
impl PressableExt for Rect {}
