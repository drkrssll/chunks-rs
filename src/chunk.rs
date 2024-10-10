use gtk4::{
    prelude::{GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Label,
};
use gtk4_layer_shell::{Edge, Layer};

use crate::Wayland;

pub struct Chunk;

impl Chunk {
    pub fn new(
        factory: &Application,
        title: &str,
        tag: Label,
        anchors: Vec<(Edge, bool)>,
        margins: Vec<(Edge, i32)>,
        layer: Layer,
    ) {
        let chunk = ApplicationWindow::builder()
            .application(factory)
            .title(title)
            .child(&tag)
            .build();

        if Wayland::detect_wayland() {
            Wayland::setup_window(&chunk, margins, anchors, layer);
        }

        chunk.set_decorated(false);
        chunk.set_resizable(false);

        chunk.present();
    }

    pub fn tag(class_name: &str) -> Label {
        let tag = Label::new(None);
        tag.set_widget_name(class_name);
        tag
    }
}
