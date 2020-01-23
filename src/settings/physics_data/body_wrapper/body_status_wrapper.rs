use deathframe::specs_physics::nphysics::object::BodyStatus;

#[derive(Clone, Deserialize)]
pub enum BodyStatusWrapper {
    Disabled,
    Static,
    Dynamic,
    Kinematic,
}

impl Into<BodyStatus> for &BodyStatusWrapper {
    fn into(self) -> BodyStatus {
        match self {
            BodyStatusWrapper::Disabled => BodyStatus::Disabled,
            BodyStatusWrapper::Static => BodyStatus::Static,
            BodyStatusWrapper::Dynamic => BodyStatus::Dynamic,
            BodyStatusWrapper::Kinematic => BodyStatus::Kinematic,
        }
    }
}
