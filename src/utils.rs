use gtk4::{
    gdk::Display, style_context_add_provider_for_display, CssProvider,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};

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

