# Chunks-rs
[![Crates.io](https://img.shields.io/crates/d/chunks-rs?style=flat-square&color=red)](https://crates.io/crates/chunks-rs)

A library that simplifies the process of making widgets for Wayland Compositors.

<div style="display: flex; align-items: center;">
    <img src="screenshots/scrot1.jpg" style="height: 150px;">
    <img src="screenshots/scrot3.jpg" style="height: 150px;">
    <img src="screenshots/scrot4.jpg" style="height: 150px;">
</div>

Chunks uses GTK4 and GTK4 Layer Shell at its core, and comes stock with a listener for the Hyprland IPC. This helps with changing Widget states when something changes, such as making the current window fullscreen.

## Usage

Make sure you have GTK4 and GTK4-Layer-Shell installed on your system.

> For more in depth examples, please refer to [example-chunks](https://github.com/drkrssll/example-chunks)

```toml
[dependencies]
chunks-rs = "0.4.8"
```

This will create a storage widget, similar to the one in the screenshot:
```rs
const STYLE: &str = "
window {
    background-color: transparent;
}

#storage {
    font-size: 34px;
    background-color: #000000;
    color: #FFFFFF;
}
";

fn main() {
    let factory = Factory::new("chunk.factory");

    let chunks = |factory: Application| {
        storage(&factory);

        load_css(STYLE);
    };

    factory.pollute(chunks);
}

fn storage(factory: &Application) {
    let tag = tag("storage");
    let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];
    let anchors = EdgeConfig::TOP_RIGHT.to_vec();

    let storage_closure = || {
        let text = format!(
            "<span foreground='#FFFFFF'>{:.0}%</span>",
            Internal::get_storage(),
        );
        text
    };

    Internal::update_storage(&tag, storage_closure);

    Chunk::new(
        factory.clone(),
        "Storage".to_string(),
        tag,
        anchors,
        margins,
        Layer::Bottom,
    )
    .build();
}
```

## Slabs

Chunks has recently had a new addition - Slabs. These are Popup Widgets, with almost the exact same implementation as Chunks.

> Slabs do not need a designated layer, as they are set to Overlay by default.
> Instead of a layer, enter the amount of seconds you would like the Popup to display for.
```rs
Slab::new(
    factory.clone(),
    "Volume".to_string(),
    tag,
    anchors,
    margins,
    2, // How long to display (In Seconds)
)
.build();
```

Slabs have very limited functionality as of right now.
