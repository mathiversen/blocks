pub mod events;
mod macros;
pub mod strings;
pub mod traits;
pub mod web_api;

pub mod prelude {
    use futures_signals::signal::Signal;
    use std::pin::Pin;

    pub type SignalReturn<A> = Pin<Box<dyn Signal<Item = A>>>;

    pub use crate::events::*;
    pub use crate::macros::*;
    pub use crate::strings::*;
    pub use crate::traits::*;
    pub use crate::web_api::*;
    pub use crate::{console_err, console_log};
}
