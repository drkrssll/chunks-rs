use gio::prelude::Cast;
use gtk4::{
    prelude::{BoxExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Orientation, Widget,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use crate::widgets::Tag;

/// The vector of tags represents text boxes with CSS class names for styling.
/// These tags are then appended to the GTK4 `Box` widget, in order to break your taskbar into
/// multiple segments.
pub struct Bar {
    factory: Application,
    title: String,
    tags: Vec<Tag>,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    orientation: Orientation,
}

impl Bar {
    pub fn new(
        factory: Application,
        title: &str,
        tags: Vec<Tag>,
        margins: Vec<(Edge, i32)>,
        anchors: Vec<(Edge, bool)>,
        orientation: Orientation,
    ) -> Self {
        Self {
            factory,
            title: title.to_string(),
            tags,
            margins,
            anchors,
            orientation,
        }
    }

    /// By default, the Bar is built with a CSS class name of "taskbar".
    /// All underlying Tags will retain their respective CSS class names, for advanced styling.
    pub fn build(&self) {
        let mut children: Vec<Widget> = Vec::new();

        for tag in &self.tags {
            let child = match tag {
                Tag::Label(label) => label.clone().upcast::<Widget>(),
                Tag::Box(box_) => box_.clone().upcast::<Widget>(),
                Tag::Button(button) => button.clone().upcast::<Widget>(),
                Tag::Revealer(revealer) => revealer.clone().upcast::<Widget>(),
            };

            children.push(child);
        }

        let bar = ApplicationWindow::builder()
            .application(&self.factory)
            .title(&self.title)
            .build();

        let gtk4_box = gtk4::Box::builder().orientation(self.orientation).build();

        gtk4_box.set_widget_name("taskbar");

        bar.init_layer_shell();
        bar.set_layer(Layer::Top);
        bar.auto_exclusive_zone_enable();

        for (edge, margin) in &self.margins {
            bar.set_margin(*edge, *margin);
        }

        for (anchor, state) in &self.anchors {
            bar.set_anchor(*anchor, *state);
        }

        for child in children {
            gtk4_box.append(&child);
        }

        bar.set_child(Some(&gtk4_box));
        bar.present();
    }
}
