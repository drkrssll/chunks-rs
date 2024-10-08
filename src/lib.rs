use std::{
    env,
    io::{BufRead, BufReader, Write},
    os::unix::net::UnixStream,
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

use chrono::Local;
use gio::{
    glib::{clone::Downgrade, timeout_add_local, ControlFlow},
    prelude::IsA,
};
use gtk4::{
    gdk::Display,
    glib::{timeout_add_seconds_local, ExitCode},
    prelude::{ApplicationExtManual, WidgetExt},
    style_context_add_provider_for_display, Application, ApplicationWindow, CssProvider, Label,
    Widget, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use sysinfo::{DiskExt, System, SystemExt};

pub struct Chunks {
    app: Application,
}

impl Chunks {
    pub fn new(application_id: &str) -> Self {
        let app = Application::builder()
            .application_id(application_id)
            .build();

        Self { app }
    }

    pub fn run(&self) -> ExitCode {
        self.app.run()
    }

    pub fn create_window(&self, title: &str, child: &impl IsA<Widget>) -> ApplicationWindow {
        ApplicationWindow::builder()
            .application(&self.app)
            .title(title)
            .child(child)
            .build()
    }

    pub fn setup_wayland_window(
        &self,
        window: &ApplicationWindow,
        margins: Vec<(Edge, i32)>,
        anchors: Vec<(Edge, bool)>,
        layout: Layer,
    ) {
        window.init_layer_shell();
        window.set_layer(layout);

        for (edge, margin) in margins {
            window.set_margin(edge, margin)
        }

        for (anchor, state) in anchors {
            window.set_anchor(anchor, state);
        }

        let (window_sender, window_receiver) = channel::<bool>();
        self.event_listener(window_sender);

        let window_weak = window.downgrade();
        timeout_add_local(Duration::from_millis(100), move || {
            if let Ok(is_fullscreen) = window_receiver.try_recv() {
                if let Some(window) = window_weak.upgrade() {
                    if is_fullscreen {
                        window.hide();
                    } else {
                        window.show();
                    }
                }
            }
            gio::glib::ControlFlow::Continue
        });
    }

    pub fn create_clock_widget(&self) -> Label {
        let clock = Label::new(None);
        clock.set_widget_name("clock_label");
        self.handle_time(&clock);
        clock
    }

    pub fn create_storage_widget(&self) -> Label {
        let storage = Label::new(None);
        storage.set_widget_name("storage_label");
        self.handle_storage(&storage);
        storage
    }

    pub fn load_css(&self, css: &str) {
        let provider = CssProvider::new();
        provider.load_from_data(css);

        if let Some(display) = Display::default() {
            style_context_add_provider_for_display(
                &display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn event_listener(&self, window_sender: Sender<bool>) {
        let instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .expect("HYPRLAND_INSTANCE_SIGNATURE not found. Is Hyprland running?");

        let socket_path = format!("/run/user/1000/hypr/{}/.socket2.sock", instance_signature);

        thread::spawn(move || {
            if let Ok(mut stream) = UnixStream::connect(&socket_path) {
                let _ = stream.write_all(b"subscribewindow\n");

                let reader = BufReader::new(stream);
                for line in reader.lines() {
                    if let Ok(event) = line {
                        if event.contains("fullscreen>>1") {
                            let _ = window_sender.send(true);
                        } else if event.contains("fullscreen>>0") {
                            let _ = window_sender.send(false);
                        }
                    }
                }
            }
        });
    }

    fn handle_time(&self, clock_label: &Label) {
        let clock_clone = clock_label.clone();

        let current_time = Local::now();
        let initial_text = format!(
            "<span background='#000000' foreground='#FFFFFF' size='large'>{}</span>\n<span foreground='#fabbc2' size='small'>{}  </span><span foreground='#FF0110' weight='bold' size='small'>{}</span>",
            current_time.format("%a").to_string(),
            current_time.format("%b").to_string(),
            current_time.format("%d").to_string(),
        );

        clock_clone.set_markup(&initial_text);

        timeout_add_seconds_local(2, move || {
            let clock_clone = clock_clone.clone();
            timeout_add_seconds_local(1, move || {
                let current_time = Local::now();

                let formatted_time = format!(
                    "<span foreground='#FFFFFF' size='large'>{}</span><span foreground='#FF0110' weight='bold' size='small'>  {}</span>\n<span foreground='#FFFFFF' size='large'> {}</span>",
                    current_time.format("%I").to_string(),
                    current_time.format("%p").to_string(),
                    current_time.format("%M").to_string(),
                );

                clock_clone.set_markup(&formatted_time);

                ControlFlow::Continue
            });

            ControlFlow::Break
        });
    }

    fn handle_storage(&self, storage_label: &Label) {
        let storage_clone = storage_label.clone();

        let update_storage_usage = move || {
            let mut system = System::new_all();
            system.refresh_disks();

            let total_space: u64 = system.disks().iter().map(|disk| disk.total_space()).sum();
            let available_space: u64 = system
                .disks()
                .iter()
                .map(|disk| disk.available_space())
                .sum();

            if total_space == 0 {
                eprintln!("Warning: Total disk space is zero. Check system.disks() output.");
                storage_clone.set_text("Disk: Error");
                return ControlFlow::Continue;
            }

            let used_percentage =
                ((total_space - available_space) as f64 / total_space as f64 * 100.0).round();

            let formatted_storage = format!(
                "<span foreground='#FF0000' size='large'>/ </span><span foreground='#FFFFFF' size='large'>{:.0}%</span>",
                used_percentage
            );

            storage_clone.set_markup(&formatted_storage);

            ControlFlow::Continue
        };

        update_storage_usage();

        timeout_add_seconds_local(60, update_storage_usage);
    }
}

pub fn detect_wayland() -> bool {
    let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or_default();

    session_type.contains("wayland")
        || (!wayland_display.is_empty() && !session_type.contains("x11"))
}
