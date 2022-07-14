use block_utils::prelude::*;
use dominator::Dom;
use std::{fmt::Debug, sync::Arc};

pub trait Component
where
    Self: Sized + Debug,
{
    type Argument;

    /// Create the component
    fn new(args: Self::Argument) -> Self;

    /// Get the name of the component
    fn get_component_name(&self) -> String {
        get_struct_name::<Self>()
    }

    /// Transform component into html element with callbacks
    fn render<F>(state: Arc<Self>, on_event: F) -> Dom
    where
        F: FnMut(Event) + 'static;

    /// Helper method to toggle visibility
    fn is_visible(&self) -> SignalReturn<bool>;
}
