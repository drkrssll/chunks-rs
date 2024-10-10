use gtk4_layer_shell::Edge;

#[derive(Debug, Clone, Copy)]
pub struct EdgeConfig {
    left: (Edge, bool),
    right: (Edge, bool),
    top: (Edge, bool),
    bottom: (Edge, bool),
}

impl EdgeConfig {
    pub const TOP_RIGHT: Self = Self {
        left: (Edge::Left, false),
        right: (Edge::Right, true),
        top: (Edge::Top, true),
        bottom: (Edge::Bottom, false),
    };

    pub const BOTTOM_RIGHT: Self = Self {
        left: (Edge::Left, false),
        right: (Edge::Right, true),
        top: (Edge::Top, false),
        bottom: (Edge::Bottom, true),
    };

    pub const TOP_LEFT: Self = Self {
        left: (Edge::Left, true),
        right: (Edge::Right, false),
        top: (Edge::Top, true),
        bottom: (Edge::Bottom, false),
    };

    pub const BOTTOM_LEFT: Self = Self {
        left: (Edge::Left, true),
        right: (Edge::Right, false),
        top: (Edge::Top, false),
        bottom: (Edge::Bottom, true),
    };

    pub const CENTER: Self = Self {
        left: (Edge::Left, false),
        right: (Edge::Right, false),
        top: (Edge::Top, false),
        bottom: (Edge::Bottom, false),
    };

    pub const TOP_CENTER: Self = Self {
        left: (Edge::Left, false),
        right: (Edge::Right, false),
        top: (Edge::Top, true),
        bottom: (Edge::Bottom, false),
    };

    pub const BOTTOM_CENTER: Self = Self {
        left: (Edge::Left, false),
        right: (Edge::Right, false),
        top: (Edge::Top, false),
        bottom: (Edge::Bottom, true),
    };

    pub fn to_vec(&self) -> Vec<(Edge, bool)> {
        vec![self.left, self.right, self.top, self.bottom]
    }
}
