# Chunks-rs

![Screenshot](screenshot.jpg)

A library that simplifies the process of making widgets for Wayland Compositors.

Chunks uses GTK4 and GTK4 Layer Shell at its core, and comes stock with a listener for the Hyprland IPC. This helps with changing Widget states when something changes, such as making the current window fullscreen.

## Usage

> For more in depth examples, please refer to [example-chunks](https://github.com/drkrssll/example-chunks)

```toml
[dependencies]
chunks-rs = "0.3.5"
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
        storage(factory);

        load_css(STYLE);
    };

    factory.pollute(chunks);
}

fn storage(factory: Application) {
    let tag = Chunk::tag("storage");

    let anchors = EdgeConfig::TOP_RIGHT.to_vec();
    let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];

    let text = format!(
        "<span>{:.0}%</span>",
        Internal::get_storage(),
    );

    Internal::update_storage(&tag, text);

    let chunk = Chunk::new(factory, "Storage".to_string(), tag).build();

    let window = Wayland::new(chunk, margins, anchors, Layer::Bottom);

    window.setup_window()
}

```


