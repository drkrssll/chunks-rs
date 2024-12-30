use std::env;

use gio::{ApplicationCommandLine, ApplicationFlags};
use gtk4::{
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
    Application,
};

/// Represents a GTK4 Application (aka Factory) for managing and running the widgets' lifecycle.
pub struct Factory {
    application: Application,
}

impl Factory {
    /// Creates a new `Factory` with the given application ID.
    pub fn new(id: &str) -> Self {
        if env::args().len() > 1 {
            let application = Application::builder()
                .application_id(id)
                .flags(ApplicationFlags::HANDLES_COMMAND_LINE)
                .build();

            Self { application }
        } else {
            let application = Application::builder().application_id(id).build();
            Self { application }
        }
    }

    /// Runs the application.
    pub fn pollute(self, chunks: impl Fn(Application) + 'static) -> ExitCode {
        self.application.connect_activate(move |app| {
            chunks(app.clone());
        });

        self.application.run();
        ExitCode::SUCCESS
    }

    /// Runs the application with arguments.
    pub fn pollute_with_args(
        self,
        handler: impl Fn(&Application, &ApplicationCommandLine) -> i32 + 'static,
        args: Vec<String>,
    ) -> ExitCode {
        // Connect the command-line signal with correct signature
        self.application.connect_command_line(handler);

        self.application.run_with_args(&args);
        ExitCode::SUCCESS
    }
}
