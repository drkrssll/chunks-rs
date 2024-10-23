#![allow(dead_code)]

mod chunk;
mod edge_conf;
mod factory;
mod internal;
mod plate;
mod slab;
mod utils;
mod wayland;

pub use chunk::Chunk;
pub use edge_conf::EdgeConfig;
pub use factory::Factory;
pub use internal::Internal;
pub use plate::Plate;
pub use slab::Slab;
pub use utils::*;
pub use wayland::Wayland;

pub use gtk4::{Application, ApplicationWindow, Label};
pub use gtk4_layer_shell::{Edge, Layer};
