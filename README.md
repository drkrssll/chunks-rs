# Chunks-rs

![Screenshot](screenshot.jpg)

A library that simplifies the process of making widgets for Unix operating systems.

Uses GTK4 and GTK4 Layer Shell at its core, for ease of use with both X11 and Wayland Window Managers.

```toml
[dependencies]
gtk4 = "0.9.2"
gtk4-layer-shell = "0.4.0"
chunks-rs = "0.1.0"
```

```rs
const STYLE: &str = "
window {
    background-color: transparent;
}

#clock_label {
    font-size: 34px;
    background-color: #000000;
    color: #FFFFFF;
}
";

fn main() -> ExitCode {
    let factory = Factory::new("chunk.factory");

    let chunks = move |factory: &Application| {
        let clock_tag = Chunk::tag("clock_label");
        let clock_margins = vec![(Edge::Top, 20), (Edge::Right, 20)];
        let clock_edge = EdgeConfig::TOP_RIGHT.to_vec();

        Internal::handle_time(&clock_tag);
        Internal::load_css(STYLE);

        Chunk::new(
            factory,
            "Clock",
            clock_tag,
            clock_margins,
            clock_edge,
            Layer::Overlay,
        );
    };

    factory.pollute(chunks)
}
```
