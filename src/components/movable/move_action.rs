use deathframe::core::geo::prelude::Axis;

#[derive(Clone, Deserialize)]
pub enum MoveAction {
    Walk {
        axis:  Axis,
        speed: f32,
    },
    Jump {
        strength: f32,
    },
    KillJump {
        strength:     f32,
        min_velocity: f32,
    },
    WallJump {
        strength: (f32, f32),
    },
    WallSlide {
        velocity: f32,
    },
    AddVelocity {
        x: Option<f32>,
        y: Option<f32>,
    },
    SetVelocity {
        x: Option<f32>,
        y: Option<f32>,
    },
}
