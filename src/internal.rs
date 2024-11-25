use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use gio::glib::ControlFlow;
use gtk4::{
    glib::timeout_add_seconds_local,
    prelude::{BoxExt, ButtonExt},
    Picture,
};
use pulsectl::controllers::{DeviceControl, SinkController};
use regex::Regex;
use sysinfo::{DiskExt, System, SystemExt};

use crate::widgets::Tag;

/// Collection of internal utilities for your widgets, including widget state management and data fetching.
pub struct Internal;

impl Internal {
    /// Sets the static text of a Tag, using markup if the text contains HTML-like tags.
    pub fn static_widget(tag: &Tag, text: String) {
        match tag {
            Tag::Label(label) => {
                if text.contains("</") && text.contains('>') {
                    label.set_markup(&text);
                } else {
                    label.set_text(&text);
                }
            }
            Tag::Button(button) => {
                button.set_label(&text);
            }
            _ => (),
        }
    }

    /// Adds a picture to a GTK4 box, given a file path.
    pub fn static_picture(tag_box: &Tag, pathname: &str) {
        if let Tag::Box(box_tag) = tag_box {
            let picture = Picture::for_filename(pathname);

            box_tag.append(&picture);
        }
    }

    /// Sets Button behavior
    pub fn static_button(tag_button: &Tag, action: impl Fn() + 'static) {
        if let Tag::Button(button) = button {
            button.connect_clicked(move |_| {
                action();
            });
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

    /// Uses pulsectl-rs to return the formatted volume level.
    pub fn get_pactl_vol() -> String {
        let mut handler = SinkController::create().unwrap();

        let devices = handler.list_devices().expect("Failed to list devices");

        if let Some(device) = devices.first() {
            let vol = device.volume.to_string();

            vol.split_whitespace()
                .find(|s| s.ends_with('%'))
                .unwrap_or("0%")
                .to_string()
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
