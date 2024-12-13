#![allow(dead_code)]

mod factory;
mod internal;
mod wayland;

pub mod position;
pub mod taskbar;
pub mod utils;
pub mod widgets;

pub use factory::Factory;
pub use internal::Internal;
pub use wayland::Wayland;

pub use gtk4::{Application as GtkApp, ApplicationWindow as GtkWindow, Orientation::*};
