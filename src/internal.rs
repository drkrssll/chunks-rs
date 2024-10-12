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

    pub fn static_to_update<F>(
        css_tag: &Label,
        format_fn: F,
        sleep: u32,
        updated_fn: F,
        interval: u32,
    ) where
        F: Fn() -> String + 'static,
    {
        let css_tag = css_tag.clone();
        let css_updater = css_tag.clone();

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
            let updated_text = updated_fn();
            let css_updater = css_updater.clone();

            timeout_add_seconds_local(interval, move || {
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
