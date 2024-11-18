use crate::chunk::Tag;

use gtk4::{
    gdk::Display, prelude::WidgetExt, style_context_add_provider_for_display, Box, CssProvider,
    Label, Orientation, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

/// Creates a new GTK4 `Label` with a specified CSS class name.
pub fn tag(class_name: &str) -> Tag {
    let tag = Label::new(None);

    tag.set_widget_name(class_name);

    Tag::Label(tag)
}

/// Creates a new GTK4 `Box` with a specified CSS class name.
pub fn tag_box(class_name: &str) -> Tag {
    let tag = Box::new(Orientation::Vertical, 0);

    tag.set_widget_name(class_name);

    Tag::Box(tag)
}

/// Loads CSS style data into the GTK4 Application.
pub fn load_css(style: &str) {
    let provider = CssProvider::new();

    provider.load_from_data(style);

    if let Some(display) = Display::default() {
        style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
