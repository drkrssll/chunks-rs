use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use gio::glib::ControlFlow;
use gtk4::{glib::timeout_add_seconds_local, Label};
use sysinfo::{DiskExt, System, SystemExt};

pub struct Internal;

impl Internal {
    pub fn static_widget(css_tag: &Label, text: String) {
        if text.contains("</") && text.contains('>') {
            css_tag.set_markup(&text);
        } else {
            css_tag.set_text(&text);
        };
    }

    pub fn get_pactl_vol() -> String {
        let output = Command::new("pactl")
            .args(&["get-sink-volume", "@DEFAULT_SINK@"])
            .output()
            .expect("Failed to execute pactl command");

        let output_str = String::from_utf8_lossy(&output.stdout);

        if let Some(volume) = output_str.split_whitespace().find(|&s| s.ends_with('%')) {
            volume.to_string()
        } else {
            "Unknown".to_string()
        }
    }

    pub fn static_to_update<F, G>(
        css_tag: &Label,
        format_fn: F,
        sleep: u32,
        updated_fn: G,
        interval: u32,
    ) where
        F: Fn() -> String + 'static,
        G: Fn() -> String + 'static,
    {
        let css_tag = css_tag.clone();
        let css_updater = css_tag.clone();
        let updated_fn = Arc::new(Mutex::new(updated_fn));

        let update = move || {
            let text = format_fn();

            if text.contains("</") && text.contains('>') {
                css_tag.set_markup(&text);
            } else {
                css_tag.set_text(&text);
            };

            ControlFlow::Continue
        };

        update();

        timeout_add_seconds_local(sleep, move || {
            let css_updater = css_updater.clone();
            let updated_fn = Arc::clone(&updated_fn);

            timeout_add_seconds_local(interval, move || {
                let updated_text = (updated_fn.lock().unwrap())();

                if updated_text.contains("</") && updated_text.contains('>') {
                    css_updater.set_markup(&updated_text);
                } else {
                    css_updater.set_text(&updated_text);
                };

                ControlFlow::Continue
            });

            ControlFlow::Break
        });
    }

    pub fn update_widget<F>(css_tag: &Label, format_fn: F, interval: u32)
    where
        F: Fn() -> String + 'static,
    {
        let css_tag = css_tag.clone();

        let update = move || {
            let text = format_fn();

            if text.contains("</") && text.contains('>') {
                css_tag.set_markup(&text);
            } else {
                css_tag.set_text(&text);
            };

            ControlFlow::Continue
        };

        update();

        timeout_add_seconds_local(interval, update);
    }

    pub fn update_time<F>(css_tag: &Label, format_fn: F)
    where
        F: Fn() -> String + 'static,
    {
        Internal::update_widget(css_tag, format_fn, 1)
    }

    pub fn update_storage<F>(css_tag: &Label, format_fn: F)
    where
        F: Fn() -> String + 'static,
    {
        Internal::update_widget(css_tag, format_fn, 120)
    }

    pub fn get_storage() -> f64 {
        let mut system = System::new_all();
        system.refresh_disks();

        let total_space: u64 = system.disks().iter().map(|disk| disk.total_space()).sum();
        let available_space: u64 = system
            .disks()
            .iter()
            .map(|disk| disk.available_space())
            .sum();

        let used_percentage =
            ((total_space - available_space) as f64 / total_space as f64 * 100.0).round();

        used_percentage
    }
}
