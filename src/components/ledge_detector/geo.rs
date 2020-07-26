use deathframe::physics::query::exp::prelude::QueryValueSide;

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

impl Into<QueryValueSide> for &LedgeDetectorSide {
    fn into(self) -> QueryValueSide {
        use self::LedgeDetectorSide as Side;

        match self {
            Side::Top => QueryValueSide::Top,
            Side::Bottom => QueryValueSide::Bottom,
            Side::Left => QueryValueSide::Left,
            Side::Right => QueryValueSide::Right,
        }
    }
}
