use dominator::Dom;
use futures_signals::signal::Signal;
use serde::Serialize;
use std::fmt::Debug;
use std::{pin::Pin, sync::Arc};

use crate::utils::get_struct_name;

pub type SignalReturn<A> = Pin<Box<dyn Signal<Item = A>>>;

pub trait Component
where
    Self: Sized + Debug + Default + Serialize,
{
    type Argument;

    /// Create the component
    fn new(args: Self::Argument) -> Self;

    /// Get the name of the component
    fn name(&self) -> String {
        get_struct_name::<Self>()
    }

    /// Transform component into html element with callbacks
    fn render(c: Arc<Self>) -> Dom;

    /// Helper method to toggle visibility
    fn is_visible(&self) -> SignalReturn<bool>;
}
