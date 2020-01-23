use super::component_prelude::*;

const DEFAULT_PADDING: f32 = 1.0;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Grounded {
    pub padding: f32,
    on_ground:   bool,
}

impl Grounded {
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn is_on_ground(&self) -> bool {
        self.on_ground
    }

    pub fn is_in_air(&self) -> bool {
        !self.on_ground
    }

    pub fn set_on_ground(&mut self, on_ground: bool) {
        self.on_ground = on_ground;
    }
}

impl Default for Grounded {
    fn default() -> Self {
        Self {
            padding:   DEFAULT_PADDING,
            on_ground: false,
        }
    }
}
