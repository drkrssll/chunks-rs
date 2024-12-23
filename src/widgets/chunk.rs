use crate::Wayland;

use gio::prelude::Cast;
use gtk4::{prelude::GtkWindowExt, Application, ApplicationWindow, Box, Button, Label, Widget};
use gtk4_layer_shell::{Edge, Layer};

pub enum Tag {
    Label(Label),
    Box(Box),
    Button(Button),
}

/// Represents a GTK4 window with a configuration for positioning/display on Wayland.
/// The tag represents a text box with a CSS class name for styling.
pub struct Chunk {
    factory: Application,
    title: String,
    tag: Tag,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    layer: Layer,
}

impl Chunk {
    /// Creates a new `Chunk` instance with the given parameters.
    pub fn new(
        factory: Application,
        title: &str,
        tag: Tag,
        margins: Vec<(Edge, i32)>,
        anchors: Vec<(Edge, bool)>,
        layer: Layer,
    ) -> Self {
        Self {
            factory,
            title: title.to_string(),
            tag,
            margins,
            anchors,
            layer,
        }
    }

    /// Builds and displays the `Chunk` window, configuring it for Wayland if detected.
    pub fn build(self) {
        let child = match self.tag {
            Tag::Label(label) => label.upcast::<Widget>(),
            Tag::Box(box_) => box_.upcast::<Widget>(),
            Tag::Button(button) => button.upcast::<Widget>(),
        };

        let chunk = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title)
            .child(&child)
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
