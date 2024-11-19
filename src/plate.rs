use crate::{chunk::Tag, Wayland};

use std::time::Duration;

use gio::{
    glib::{clone::Downgrade, timeout_add_local, ControlFlow},
    prelude::Cast,
};
use gtk4::{prelude::GtkWindowExt, Application, ApplicationWindow, Widget};
use gtk4_layer_shell::{Edge, Layer};

pub struct Plate {
    factory: Application,
    title: String,
    tag: Tag,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    duration: u64,
}

impl Plate {
    /// Creates a new `Plate` with the given parameters.
    pub fn new(
        factory: Application,
        title: String,
        tag: Tag,
        margins: Vec<(Edge, i32)>,
        anchors: Vec<(Edge, bool)>,
        duration: u64,
    ) -> Self {
        Self {
            factory,
            title,
            tag,
            margins,
            anchors,
            duration,
        }
    }

    /// Builds and displays the `Plate` window, which will close automatically after a set duration.
    /// Perfect for greeter widgets.
    pub fn build(self) {
        let child = match self.tag {
            Tag::Label(label) => label.upcast::<Widget>(),
            Tag::Box(box_) => box_.upcast::<Widget>(),
        };

        let plate = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title)
            .child(&child)
            .build();

        if Wayland::detect_wayland() {
            let wayland = Wayland::new(plate.clone(), self.anchors, self.margins, Layer::Overlay);

            wayland.setup_window()
        }

        plate.set_decorated(false);
        plate.set_resizable(false);

        plate.present();

        let duration = Duration::from_secs(self.duration);

        let plate_weak = plate.downgrade();
        timeout_add_local(duration, move || {
            if let Some(window) = plate_weak.upgrade() {
                window.destroy();
            }
            ControlFlow::Break
        });
    }
}
