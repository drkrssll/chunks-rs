# Chunks-rs

![Screenshot](screenshot.jpg)

A library that simplifies the process of making widgets for Unix operating systems.

Uses GTK4 and GTK4 Layer Shell at its core, for ease of use with both X11 and Wayland Window Managers.

```toml
[dependencies]
chunks-rs = "0.3.2"
```

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

    factory.pollute(move |factory: &Application| {
        let storage = format!(
            "Disk: {}",
            get_storage()
        );

        let title = "Storage Example";
        let tag = Chunk::tag("storage");

        let anchors = EdgeConfig::TOP_RIGHT.to_vec();
        let margins = vec![(Edge::Top, 20), (Edge::Right, 20)];

        Internal::update_storage(&tag, storage);
        Internal::load_css(STYLE);

        Chunk::new(
            factory,
            title,
            tag,
            anchors,
            margins,
            Layer::Overlay,
        );
    });
}
```
