use deathframe::core::geo::prelude::Axis;

#[derive(Clone, Deserialize)]
pub enum MoveAction {
    /// Requires component `MoveAcceleration`.
    /// Walk along the given axis, with the given `mult` as the
    /// direction multiplier, with the velocity from `MoveAcceleration`.
    Walk {
        axis: Axis,
        mult: f32,
    },

    /// Requires component `Jumper`.
    /// Jumps with the jump strength from `Jumper`.
    Jump,

    /// Requires component `Jumper`.
    /// Kills the jump with the kill strength
    /// and the min velocity from `Jumper`.
    KillJump,

    /// Requires component `Jumper` and `WallJumper`.
    /// Does a wall jump with the strength from `WallJumper`.
    /// Jumps in the x direction given by `x_mult`.
    WallJump {
        x_mult: f32,
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
