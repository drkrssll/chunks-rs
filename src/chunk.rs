use gtk4::{
    prelude::{GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Label,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

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

        LayerShell::init_layer_shell(&chunk);

        chunk.set_layer(layer);

        for (edge, margin) in &margins {
            chunk.set_margin(*edge, *margin)
        }

        for (anchor, state) in &anchors {
            chunk.set_anchor(*anchor, *state);
        }

        if Wayland::detect_wayland() {
            Wayland::setup_window(&chunk);
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
