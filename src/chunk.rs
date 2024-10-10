use gtk4::{
    prelude::{GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Label,
};

pub struct Chunk {
    factory: Application,
    title: String,
    tag: Label,
}

impl Chunk {
    pub fn new(self) -> ApplicationWindow {
        let chunk = ApplicationWindow::builder()
            .application(&self.factory)
            .title(self.title)
            .child(&self.tag)
            .build();

        chunk.set_decorated(false);
        chunk.set_resizable(false);

        chunk
    }

    pub fn tag(class_name: &str) -> Label {
        let tag = Label::new(None);
        tag.set_widget_name(class_name);
        tag
    }
}
