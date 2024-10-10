# Chunks-rs

![Screenshot](screenshot.jpg)

A library that simplifies the process of making widgets for Unix operating systems.

Uses GTK4 and GTK4 Layer Shell at its core, for ease of use with both X11 and Wayland Window Managers.

```toml
[dependencies]
chunks-rs = "0.3.0"
```

```rs
const STYLE: &str = "
window {
    background-color: transparent;
}

#clock {
    font-size: 34px;
    background-color: #000000;
    color: #FFFFFF;
}
";

fn main() {
    let factory = Factory::new("chunk.factory");

    factory.pollute(move |factory: &Application| {
        let time = Local::now();
        let formatted_time = format!(
            "{}:{}"
            time.format("%I").to_string(),
            time.format("%M").to_string(),
        );

        let title = "Clock Example";
        let tag = Chunk::tag("clock");

        let anchors = EdgeConfig::TOP_RIGHT.to_vec();
        let margins = vec![(Edge::Top, 20), (Edge::Right, 20)];

        Internal::handle_time(&tag, formatted_time);
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
