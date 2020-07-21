#[derive(Clone, Deserialize)]
pub enum LedgeDetectorCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Deserialize)]
pub enum LedgeDetectorSide {
    Top,
    Bottom,
    Left,
    Right,
}
