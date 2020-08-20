use super::{Object, Pos, Props, Tile};

pub trait Propful {
    fn props(&self) -> &Props;

    fn pos(&self) -> Pos;

    fn default_z() -> f32;

    fn z(&self) -> Option<f32> {
        self.props()
            .get("z")
            .and_then(|val| val.as_f64())
            .map(|f| f as f32)
    }

    fn z_or_default(&self) -> f32 {
        self.z().unwrap_or_else(Self::default_z)
    }

    fn is_solid(&self) -> bool {
        self.props()
            .get("solid")
            .and_then(|prop| prop.as_bool())
            .unwrap_or(false)
    }

    fn variant(&self) -> Option<String> {
        self.props()
            .get("variant")
            .and_then(|val| val.as_str())
            .map(ToString::to_string)
    }
}

impl Propful for Tile {
    fn props(&self) -> &Props {
        &self.props
    }
    fn pos(&self) -> Pos {
        self.pos
    }
    fn default_z() -> f32 {
        0.0
    }
}

impl Propful for Object {
    fn props(&self) -> &Props {
        &self.props
    }
    fn pos(&self) -> Pos {
        self.pos
    }
    fn default_z() -> f32 {
        1.0
    }
}
