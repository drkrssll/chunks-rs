use std::time::Duration;

use gio::glib::{clone::Downgrade, timeout_add_local, ControlFlow};
use gtk4::{prelude::GtkWindowExt, Application, ApplicationWindow, Label};
use gtk4_layer_shell::{Edge, Layer};

use crate::Wayland;

pub struct Slab {
    factory: Application,
    title: String,
    tag: Label,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    duration: u64,
}

impl Slab {
    /// Creates a new `Slab` with the given parameters.
    pub fn new(
        factory: Application,
        title: String,
        tag: Label,
        anchors: Vec<(Edge, bool)>,
        margins: Vec<(Edge, i32)>,
        duration: u64,
    ) -> Self {
        Self {
            factory,
            title,
            tag,
            anchors,
            margins,
            duration,
        }
    }

    /// Builds and displays the `Slab` window, which will close automatically after a set duration.
    pub fn build(self) {
        let slab = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title)
            .child(&self.tag)
            .build();

        if Wayland::detect_wayland() {
            let wayland = Wayland::new(slab.clone(), self.anchors, self.margins, Layer::Overlay);

            wayland.setup_window()
        }

        slab.set_decorated(false);
        slab.set_resizable(false);

        slab.present();

        let duration = Duration::from_secs(self.duration);

        let slab_weak = slab.downgrade();
        timeout_add_local(duration, move || {
            if let Some(window) = slab_weak.upgrade() {
                window.destroy();
            }
            ControlFlow::Break
        });
    }
}
