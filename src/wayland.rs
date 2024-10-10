use std::{
    env,
    io::{BufRead, BufReader, Write},
    os::unix::net::UnixStream,
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

use gio::glib::{clone::Downgrade, timeout_add_local};
use gtk4::{prelude::WidgetExt, ApplicationWindow};
use gtk4_layer_shell::LayerShell;
pub use gtk4_layer_shell::{Edge, Layer};

pub struct Wayland;

impl Wayland {
    pub fn setup_window(
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
        Wayland::hypr_ipc(window_sender);

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

    pub fn detect_wayland() -> bool {
        let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
        let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or_default();

        session_type.contains("wayland")
            || (!wayland_display.is_empty() && !session_type.contains("x11"))
    }

    pub fn hypr_ipc(window_sender: Sender<bool>) {
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
}
