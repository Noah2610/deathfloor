use deathframe::specs_physics;
use specs_physics::nphysics::object::Ground;

#[derive(Clone, Deserialize)]
pub struct GroundWrapper;

impl Into<Ground<f32>> for &GroundWrapper {
    fn into(self) -> Ground<f32> {
        Ground::new()
    }
}
