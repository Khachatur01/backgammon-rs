use cursive::event::{Event, EventResult};
use cursive::view::ViewWrapper;
use cursive::View;


pub struct EventHandlerView<T: View> {
    view: T,
    event_handler: Box<dyn Fn(Event, &mut T) -> () + Send + Sync + 'static>,
}

impl<T: View> EventHandlerView<T> {
    pub fn new<F>(view: T, event_handler: F) -> Self
    where F: Fn(Event, &mut T) -> () + Send + Sync + 'static {
        Self {
            view,
            event_handler: Box::new(event_handler),
        }
    }
}

impl<T: View> ViewWrapper for EventHandlerView<T> {
    type V = T;

    fn with_view<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Self::V) -> R,
    {
        Some(f(&self.view))
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Self::V) -> R,
    {
        Some(f(&mut self.view))
    }

    fn into_inner(self) -> Result<Self::V, Self>
    where
        Self::V: Sized,
    {
        Ok(self.view)
    }

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        (self.event_handler)(event, &mut self.view);
        EventResult::Ignored
    }
}
