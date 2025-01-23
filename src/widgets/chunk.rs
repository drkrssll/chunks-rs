use std::cell::RefCell;

use crate::{Builder, Wayland};

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
pub struct Chunk {
    factory: Application,
    title: String,
    tag: Tag,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    layer: Layer,
    resize: bool,
    chunk: RefCell<Option<ApplicationWindow>>,
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
            chunk: None.into(),
        }
    }

    pub fn set_dimensions(&self, width: u32, height: u32) {
        if let Some(chunk) = self.chunk.borrow().as_ref() {
            chunk.set_default_size(width as i32, height as i32);
        } else {
            eprintln!("Error: Chunk has not been built yet!");
        }
    }
}

impl Builder for Chunk {
    fn build(self) {
        let child = match self.tag {
            Tag::Label(label) => label.upcast::<Widget>(),
            Tag::Box(box_) => box_.upcast::<Widget>(),
            Tag::Button(button) => button.upcast::<Widget>(),
            Tag::Revealer(revealer) => revealer.upcast::<Widget>(),
            Tag::Scroller(scroller) => scroller.upcast::<Widget>(),
            Tag::Undefined => panic!("Tag is undefined!"),
        };

        let chunk = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title)
            .child(&child)
            .resizable(self.resize)
            .build();

        if Wayland::detect_wayland() {
            let wayland = Wayland::new(chunk.clone(), self.anchors, self.margins, self.layer);
            wayland.setup_window()
        }

        chunk.set_decorated(false);
        chunk.present();

        *self.chunk.borrow_mut() = Some(chunk);
    }
}
