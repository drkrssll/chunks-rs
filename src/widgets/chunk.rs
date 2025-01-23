use crate::Wayland;

use gio::prelude::Cast;
use gtk4::{
    prelude::GtkWindowExt, Application, ApplicationWindow, Box, Button, Label, Revealer,
    ScrolledWindow, Widget,
};
use gtk4_layer_shell::{Edge, Layer};

#[derive(Clone)]
pub enum Tag {
    Label(Label),
    Box(Box),
    Button(Button),
    Revealer(Revealer),
    Scroller(ScrolledWindow),
    Undefined,
}

/// Represents a GTK4 window with a configuration for positioning/display on Wayland.
/// The tag represents a text box with a CSS class name for styling.
#[derive(Clone)]
pub struct Chunk {
    factory: Application,
    title: String,
    tag: Tag,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    layer: Layer,
    resize: bool,
    chunk: Option<ApplicationWindow>,
}

impl Chunk {
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
            chunk: None,
        }
    }

    pub fn set_dimensions(&self, width: u32, height: u32) {
        if let Some(chunk) = &self.chunk {
            chunk.set_default_size(width as i32, height as i32);
        }
    }

    /// Different from the `Plate` and `Bar` builders, the `Chunk` build() returns Self for
    /// subsequent method chaining.
    pub fn build(mut self) -> Self {
        let child = match self.tag {
            Tag::Label(ref label) => label.clone().upcast::<Widget>(),
            Tag::Box(ref box_) => box_.clone().upcast::<Widget>(),
            Tag::Button(ref button) => button.clone().upcast::<Widget>(),
            Tag::Revealer(ref revealer) => revealer.clone().upcast::<Widget>(),
            Tag::Scroller(ref scroller) => scroller.clone().upcast::<Widget>(),
            Tag::Undefined => panic!("Tag is undefined!"),
        };

        let chunk = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title.clone())
            .child(&child)
            .resizable(self.resize)
            .build();

        if Wayland::detect_wayland() {
            let wayland = Wayland::new(
                chunk.clone(),
                self.anchors.clone(),
                self.margins.clone(),
                self.layer,
            );
            wayland.setup_window()
        }

        chunk.set_decorated(false);
        chunk.present();

        self.chunk = Some(chunk);
        self
    }
}
