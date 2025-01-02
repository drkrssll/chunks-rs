use crate::widgets::Tag;

use gio::prelude::Cast;
use gtk4::{
    gdk::Display,
    prelude::{BoxExt, WidgetExt},
    style_context_add_provider_for_display, Align, Box, CssProvider, Label, Orientation, Revealer,
    RevealerTransitionType, Widget, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

/// Creates a new GTK4 `Label` with a specified CSS class name.
pub fn tag_label(class_name: &str) -> Tag {
    let tag = Label::new(None);

    tag.set_widget_name(class_name);

    Tag::Label(tag)
}

/// Creates a new GTK4 `Box` with a specified CSS class name.
/// Can be used for images.
pub fn tag_box(class_name: &str) -> Tag {
    let tag = Box::new(Orientation::Vertical, 0);

    tag.set_widget_name(class_name);

    Tag::Box(tag)
}

/// Creates a new GTK4 `Box` with a specified CSS class name, orientation and spacing.
/// Used for grouping widgets together.
pub fn tag_container(
    class_name: &str,
    orientation: Orientation,
    spacing: i32,
    widgets: Vec<Tag>,
) -> Tag {
    let tag = Box::new(orientation, spacing);

    tag.set_widget_name(class_name);

    let widgets: Vec<Widget> = widgets
        .into_iter()
        .map(|tag| match tag {
            Tag::Label(label) => label.clone().upcast::<Widget>(),
            Tag::Box(box_) => box_.clone().upcast::<Widget>(),
            Tag::Button(button) => button.clone().upcast::<Widget>(),
            Tag::Revealer(revealer) => revealer.clone().upcast::<Widget>(),
            Tag::Undefined => panic!("Tag is undefined!"),
        })
        .collect();

    for widget in widgets {
        tag.append(&widget);
    }

    Tag::Box(tag)
}

/// Creates a new GTK4 `Button` with a specified CSS class name.
pub fn tag_button(class_name: &str) -> Tag {
    let tag = gtk4::Button::new();

    tag.set_widget_name(class_name);

    Tag::Button(tag)
}

pub fn tag_revealer(
    class_name: &str,
    child: Tag,
    duration: u32,
    transition: RevealerTransitionType,
) -> Tag {
    let tag = Revealer::new();
    tag.set_transition_duration(duration);
    tag.set_transition_type(transition);

    tag.set_widget_name(class_name);
    match child {
        Tag::Box(box_) => tag.set_child(Some(&box_)),
        Tag::Label(label) => tag.set_child(Some(&label)),
        Tag::Button(button) => tag.set_child(Some(&button)),
        Tag::Revealer(revealer) => tag.set_child(Some(&revealer)),
        _ => return Tag::Undefined,
    }

    Tag::Revealer(tag)
}

/// Positions a GTK4 `Tag` (for use inside of Bar)
// private because it doesnt work? (or i dont know what im doing)
// fuck it, ill just use CSS for this shit anyways
fn tag_position(tag: &Tag, x: Align, y: Align) {
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
        Tag::Revealer(revealer) => {
            revealer.set_halign(x);
            revealer.set_valign(y);
        }
        Tag::Undefined => {
            panic!("Tag is undefined!");
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
