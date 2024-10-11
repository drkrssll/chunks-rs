use gtk4::{
    gdk::Display, prelude::WidgetExt, style_context_add_provider_for_display, CssProvider, Label,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};

pub fn tag(class_name: &str) -> Label {
    let tag = Label::new(None);
    tag.set_widget_name(class_name);

    tag
}

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
