#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgeDetectorCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgeDetectorSide {
    Top,
    Bottom,
    Left,
    Right,
}
