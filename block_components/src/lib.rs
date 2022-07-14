mod components;
mod traits;

pub fn hello_world() -> String {
    "Hello world".into()
}

pub mod prelude {
    pub use crate::components::*;
    pub use crate::traits::*;
    pub use block_utils::prelude::*;
}
