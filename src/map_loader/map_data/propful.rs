use super::{Object, Props, Tile};

pub trait Propful {
    fn props(&self) -> &Props;

    fn z(&self) -> f32 {
        self.z_or(0.0)
    }

    fn z_or(&self, default: f32) -> f32 {
        self.props()
            .get("z")
            .and_then(|z_value| z_value.as_f64().map(|f| f as f32))
            .unwrap_or(default)
    }

    fn is_solid(&self) -> bool {
        self.props()
            .get("solid")
            .and_then(|prop| prop.as_bool())
            .unwrap_or(false)
    }
}

impl Propful for Tile {
    fn props(&self) -> &Props {
        &self.props
    }
}

impl Propful for Object {
    fn props(&self) -> &Props {
        &self.props
    }
}
