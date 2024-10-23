use std::time::Duration;

use gio::{
    glib::{timeout_add_local, ControlFlow},
    prelude::ObjectExt,
};
use gtk4::{prelude::GtkWindowExt, prelude::WidgetExt, Application, ApplicationWindow, Label};
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

    /// Builds and displays the `Slab` window, which will show whenever the text changes.
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

        slab.hide();

        let duration = Duration::from_secs(self.duration);
        let slab_weak = slab.downgrade();

        self.tag
            .connect_notify_local(Some("label"), move |_label, _| {
                if let Some(window) = slab_weak.upgrade() {
                    window.present();
                    window.set_visible(true);

                    let window_weak = window.downgrade();
                    timeout_add_local(duration, move || {
                        if let Some(window) = window_weak.upgrade() {
                            window.hide();
                        }
                        ControlFlow::Break
                    });
                }
            });
    }
}
