use std::{
    error::Error,
    process::Command,
    sync::{Arc, Mutex},
};

use gio::glib::ControlFlow;
use gtk4::{
    glib::timeout_add_seconds_local, prelude::BoxExt, prelude::WidgetExt, Box as GtkBox, Label,
    Picture,
};
use regex::Regex;
use sysinfo::{DiskExt, System, SystemExt};

use crate::chunk::Tag;

/// Collection of internal utilities for your widgets, including widget state management and data fetching.
pub struct Internal;

impl Internal {
    /// Sets the static text of a Widget, using markup if the text contains HTML-like tags.
    pub fn static_widget(tag: &Tag, text: String) {
        if let Tag::Label(label) = tag {
            if text.contains("</") && text.contains('>') {
                label.set_markup(&text);
            } else {
                label.set_text(&text);
            }
        }
    }

    /// Adds a picture to a GTK4 box, given a file path.
    pub fn static_picture(tag: &Tag, pathname: &str) {
        if let Tag::Box(box_tag) = tag {
            let picture = Picture::for_filename(pathname);

            box_tag.append(&picture);
        }
    }

    /// Fetches the weather for a given location using the wttr.in API.
    pub async fn get_weather(location: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("https://wttr.in/{}?format=3", location);
        let response = reqwest::get(&url).await?.text().await?;

        let regex = Regex::new(r"\s*([\d]+°F)")?;

        if let Some(caps) = regex.captures(&response) {
            let emoji = caps.get(1).map_or("", |m| m.as_str());
            let temperature = caps.get(2).map_or("", |m| m.as_str());

            Ok(format!("{} {}", emoji, temperature).trim().to_string())
        } else {
            Ok("Weather data not available".to_string())
        }
    }

    /// Fetches the current volume level using the `pactl` command.
    pub fn get_pactl_vol() -> String {
        let output = Command::new("pactl")
            .args(["get-sink-volume", "@DEFAULT_SINK@"])
            .output()
            .expect("Failed to execute pactl command");

        let output_str = String::from_utf8_lossy(&output.stdout);

        if let Some(volume) = output_str.split_whitespace().find(|&s| s.ends_with('%')) {
            volume.to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// Sets static text and then updates it at a given interval using a closure.
    pub fn static_to_update<F, G>(tag: &Tag, format_fn: F, sleep: u32, updated_fn: G, interval: u32)
    where
        F: Fn() -> String + 'static,
        G: Fn() -> String + 'static,
    {
        if let Tag::Label(label) = tag {
            let css_tag = label.clone();
            let css_updater = css_tag.clone();
            let updated_fn = Arc::new(Mutex::new(updated_fn));

            let update = move || {
                let text = format_fn();

                if text.contains("</") && text.contains('>') {
                    css_tag.set_markup(&text);
                } else {
                    css_tag.set_text(&text);
                }

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
                    }

                    ControlFlow::Continue
                });

                ControlFlow::Break
            });
        }
    }

    /// Updates a GTK4 label using a closure at a given interval.
    pub fn update_widget<F>(tag: &Tag, format_fn: F, interval: u32)
    where
        F: Fn() -> String + 'static,
    {
        if let Tag::Label(label) = tag {
            let css_tag = label.clone();

            let update = move || {
                let text = format_fn();

                if text.contains("</") && text.contains('>') {
                    css_tag.set_markup(&text);
                } else {
                    css_tag.set_text(&text);
                }

                ControlFlow::Continue
            };

            update();

            timeout_add_seconds_local(interval, update);
        }
    }

    /// Updates the time in a GTK4 label every second given a closure.
    pub fn update_time<F>(tag: &Tag, format_fn: F)
    where
        F: Fn() -> String + 'static,
    {
        Internal::update_widget(tag, format_fn, 1);
    }

    /// Updates the storage in a GTK4 label every 2 minutes given a closure.
    pub fn update_storage<F>(tag: &Tag, format_fn: F)
    where
        F: Fn() -> String + 'static,
    {
        Internal::update_widget(tag, format_fn, 120);
    }
    /// Fetches the current storage usage as a percentage.
    pub fn get_storage() -> f64 {
        let mut system = System::new_all();
        system.refresh_disks();

        let total_space: u64 = system.disks().iter().map(|disk| disk.total_space()).sum();
        let available_space: u64 = system
            .disks()
            .iter()
            .map(|disk| disk.available_space())
            .sum();

        ((total_space - available_space) as f64 / total_space as f64 * 100.0).round()
    }
}
