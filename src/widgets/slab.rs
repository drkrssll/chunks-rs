use std::time::Duration;

use gio::{
    glib::{timeout_add_local, ControlFlow, MainContext},
    prelude::{Cast, ObjectExt},
};
use gtk4::{
    prelude::{GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Widget,
};
use gtk4_layer_shell::{Edge, Layer};

use crate::{widgets::Tag, Builder, Wayland};

pub struct Slab {
    factory: Application,
    title: String,
    tag: Tag,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    duration: u64,
}

impl Slab {
    /// Creates a new `Slab` with the given parameters.
    pub fn new(
        factory: Application,
        title: &str,
        tag: Tag,
        margins: Vec<(Edge, i32)>,
        anchors: Vec<(Edge, bool)>,
        duration: u64,
    ) -> Self {
        Self {
            factory,
            title: title.to_string(),
            tag,
            margins,
            anchors,
            duration,
        }
    }

    fn process_events(depth: u32, max_depth: u32) {
        if depth >= max_depth {
            return;
        }

        if MainContext::default().iteration(false) {
            Self::process_events(depth + 1, max_depth);
        }
    }

    fn handle_visibility(window: &ApplicationWindow, visible: bool) {
        if visible {
            window.present();
            window.set_visible(true);
        } else {
            window.hide();
        }
        Self::process_events(0, 10);
    }
}

impl Builder for Slab {
    /// Builds and displays the `Slab` window, which will show whenever the text changes.
    fn build(self) {
        let child = match self.tag {
            Tag::Label(label) => label.upcast::<Widget>(),
            Tag::Box(box_) => box_.upcast::<Widget>(),
            Tag::Button(button) => button.upcast::<Widget>(),
            Tag::Revealer(revealer) => revealer.upcast::<Widget>(),
            Tag::Undefined => panic!("Tag is undefined!"),
        };

        let slab = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title.clone())
            .child(&child)
            .build();

        if Wayland::detect_wayland() {
            let wayland = Wayland::new(slab.clone(), self.anchors, self.margins, Layer::Overlay);
            wayland.setup_window()
        }

        slab.set_decorated(false);
        slab.set_resizable(false);
        slab.hide();

        Wayland::ipc_ignore_window(&self.title);

        let duration = Duration::from_secs(self.duration);
        let slab_weak = slab.downgrade();

        child.connect_notify_local(Some("label"), move |_label, _| {
            if let Some(window) = slab_weak.upgrade() {
                Self::handle_visibility(&window, true);

                let window_weak = window.downgrade();
                timeout_add_local(duration, move || {
                    if let Some(window) = window_weak.upgrade() {
                        Self::handle_visibility(&window, false);
                    }
                    ControlFlow::Break
                });
            }
        });
    }
}
