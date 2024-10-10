use gtk4::{
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
    Application,
};

pub struct Factory {
    application: Application,
}

impl Factory {
    pub fn new(id: &str) -> Self {
        gtk4::init().expect("Failed to initialize GTK");
        let application = Application::builder().application_id(id).build();
        Self { application }
    }

    pub fn pollute(&self, chunks: impl Fn(&Application) + 'static) -> ExitCode {
        self.application.connect_activate(move |app| {
            chunks(app);
        });
        self.application.run();
        ExitCode::SUCCESS
    }
}
