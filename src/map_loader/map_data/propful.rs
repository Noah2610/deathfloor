use super::{Object, Props, Tile};

pub trait Propful {
    fn props(&self) -> &Props;

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
