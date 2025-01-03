use std::{
    collections::{HashMap, HashSet},
    env,
    io::{BufRead, BufReader, Result, Write},
    os::unix::net::UnixStream,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use gio::glib::{timeout_add_local, ControlFlow};
use gtk4::{prelude::*, ApplicationWindow};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use once_cell::sync::Lazy;

static IGNORED_WINDOWS: Lazy<Arc<Mutex<HashSet<String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashSet::new())));

pub struct Wayland {
    chunk: ApplicationWindow,
    margins: Vec<(Edge, i32)>,
    anchors: Vec<(Edge, bool)>,
    layer: Layer,
}

impl Wayland {
    pub fn new(
        chunk: ApplicationWindow,
        anchors: Vec<(Edge, bool)>,
        margins: Vec<(Edge, i32)>,
        layer: Layer,
    ) -> Self {
        Self {
            chunk,
            anchors,
            margins,
            layer,
        }
    }

    /// Public function to add a window title to the ignore list
    pub fn ipc_ignore_window(title: &str) {
        if let Ok(mut ignored) = IGNORED_WINDOWS.lock() {
            ignored.insert(title.to_string());
        }
    }

    fn should_show_window(window: &ApplicationWindow) -> bool {
        if let Ok(ignored) = IGNORED_WINDOWS.lock() {
            if let Some(title) = window.title() {
                return !ignored.contains(&title.to_string());
            }
        }
        true
    }

    pub fn setup_window(self) {
        self.chunk.init_layer_shell();
        self.chunk.set_layer(self.layer);

        for (edge, margin) in self.margins {
            self.chunk.set_margin(edge, margin);
        }

        for (anchor, state) in self.anchors {
            self.chunk.set_anchor(anchor, state);
        }

        let (sender, receiver) = channel::<bool>();
        Self::hypr_ipc(sender);

        let chunk = self.chunk.downgrade();
        timeout_add_local(Duration::from_millis(100), move || {
            if let Ok(is_fullscreen) = receiver.try_recv() {
                if let Some(window) = chunk.upgrade() {
                    if Self::should_show_window(&window) {
                        if is_fullscreen {
                            window.hide();
                        } else {
                            window.show();
                        }
                    }
                }
            }
            ControlFlow::Continue
        });
    }

    pub fn detect_wayland() -> bool {
        let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
        let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or_default();

        session_type.contains("wayland")
            || (!wayland_display.is_empty() && !session_type.contains("x11"))
    }

    fn hypr_ipc(window_sender: Sender<bool>) {
        let instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .expect("HYPRLAND_INSTANCE_SIGNATURE not found. Is Hyprland running?");

        let socket_path = format!("/run/user/1000/hypr/{}/.socket2.sock", instance_signature);

        thread::spawn(move || {
            if let Ok(mut stream) = UnixStream::connect(&socket_path) {
                let _ = stream.write_all(b"subscribewindow\n");

                let reader = BufReader::new(stream);
                let mut fullscreen_status: HashMap<String, bool> = HashMap::new();
                let mut current_workspace = String::new();

                for line in reader.lines().map_while(Result::ok) {
                    match line {
                        _ if line.contains("fullscreen>>1") => {
                            fullscreen_status.insert(current_workspace.clone(), true);
                            let _ = window_sender.send(true);
                        }
                        _ if line.contains("fullscreen>>0") => {
                            fullscreen_status.insert(current_workspace.clone(), false);
                            let _ = window_sender.send(false);
                        }
                        _ if line.starts_with("workspace>>")
                            || line.starts_with("workspacev2>>") =>
                        {
                            let workspace_name = line.split(">>").nth(1).unwrap_or("").to_string();
                            current_workspace.clone_from(&workspace_name);

                            let is_fullscreen =
                                *fullscreen_status.get(&current_workspace).unwrap_or(&false);
                            let _ = window_sender.send(is_fullscreen);
                        }
                        _ => continue,
                    }
                }
            }
        });
    }
}
