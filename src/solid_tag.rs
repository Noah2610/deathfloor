use deathframe::components::solid::SolidTag as STag;

#[derive(Clone)]
pub enum SolidTag {
    Player,
    Platform,
}

impl STag for SolidTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (SolidTag::Player, SolidTag::Platform)
            | (SolidTag::Platform, SolidTag::Player) => true,
            (SolidTag::Player, SolidTag::Player) => true,
            (SolidTag::Platform, SolidTag::Platform) => true,
        }
    }
}

impl Default for SolidTag {
    fn default() -> Self {
        SolidTag::Platform
    }
}
