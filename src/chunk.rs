use gtk4::{prelude::GtkWindowExt, Application, ApplicationWindow, Label};
use gtk4_layer_shell::{Edge, Layer};

use crate::Wayland;

/// Represents a GTK4 window with a configuration for positioning/display on Wayland.
/// The tag represents a text box with a CSS class name for styling.
pub struct Chunk {
    factory: Application,
    title: String,
    tag: Label,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    layer: Layer,
}

impl Chunk {
    /// Creates a new `Chunk` instance with the given parameters.
    pub fn new(
        factory: Application,
        title: String,
        tag: Label,
        anchors: Vec<(Edge, bool)>,
        margins: Vec<(Edge, i32)>,
        layer: Layer,
    ) -> Self {
        Self {
            factory,
            title,
            tag,
            anchors,
            margins,
            layer,
        }
    }

    /// Builds and displays the `Chunk` window, configuring it for Wayland if detected.
    pub fn build(self) {
        let chunk = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title)
            .child(&self.tag)
            .build();

        if Wayland::detect_wayland() {
            let wayland = Wayland::new(chunk.clone(), self.anchors, self.margins, self.layer);

            wayland.setup_window()
        }

        chunk.set_decorated(false);
        chunk.set_resizable(false);

        chunk.present()
    }
}
