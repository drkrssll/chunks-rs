use crate::{Builder, Wayland};

use gio::prelude::Cast;
use gtk4::{
    prelude::GtkWindowExt, Application, ApplicationWindow, Box, Button, Label, Revealer, Widget,
};
use gtk4_layer_shell::{Edge, Layer};

#[derive(Clone)]
pub enum Tag {
    Label(Label),
    Box(Box),
    Button(Button),
    Revealer(Revealer),
    Undefined,
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
    resize: bool,
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
        resize: bool,
    ) -> Self {
        Self {
            factory,
            title: title.to_string(),
            tag,
            margins,
            anchors,
            layer,
            resize,
        }
    }
}

impl Builder for Chunk {
    /// Builds and displays the `Chunk` window, configuring it for Wayland if detected.
    fn build(self) {
        let child = match self.tag {
            Tag::Label(label) => label.upcast::<Widget>(),
            Tag::Box(box_) => box_.upcast::<Widget>(),
            Tag::Button(button) => button.upcast::<Widget>(),
            Tag::Revealer(revealer) => revealer.upcast::<Widget>(),
            Tag::Undefined => panic!("Tag is undefined!"),
        };

        let chunk = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title)
            .child(&child)
            .resizable(self.resize)
            .build();

        chunk.set_default_size(1, 1);
        if Wayland::detect_wayland() {
            let wayland = Wayland::new(chunk.clone(), self.anchors, self.margins, self.layer);

            wayland.setup_window()
        }

        chunk.set_decorated(false);

        chunk.present()
    }
}
