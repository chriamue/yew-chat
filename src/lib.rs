#[cfg(feature = "yew")]
mod components;
mod model;
#[cfg(feature = "server")]
pub mod server;

pub mod prelude {
    #[cfg(feature = "yew")]
    pub use crate::components::*;
    pub use crate::model::*;
}
