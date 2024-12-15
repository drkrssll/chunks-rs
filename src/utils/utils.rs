use crate::widgets::Tag;

use gtk4::{
    gdk::Display, prelude::WidgetExt, style_context_add_provider_for_display, Align, Box,
    CssProvider, Label, Orientation, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

/// Creates a new GTK4 `Label` with a specified CSS class name.
pub fn tag_label(class_name: &str) -> Tag {
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

/// Creates a new GTK4 `Button` with a specified CSS class name.
pub fn tag_button(class_name: &str) -> Tag {
    let tag = gtk4::Button::new();

    tag.set_widget_name(class_name);

    Tag::Button(tag)
}

/// Positions a GTK4 `Tag` (for use inside of Bar)
pub fn tag_position(tag: Tag, x: Align, y: Align) {
    match tag {
        Tag::Label(label) => {
            label.set_halign(x);
            label.set_valign(y);
        }
        Tag::Box(box_) => {
            box_.set_halign(x);
            box_.set_valign(y);
        }
        Tag::Button(button) => {
            button.set_halign(x);
            button.set_valign(y);
        }
    }
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
