use super::component_prelude::*;
use climer::{Time, Timer};
use std::time::Duration;

#[derive(Component, Default, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields, from = "DeathAfterDelayDeser")]
pub struct DeathAfterDelay {
    pub timer: Timer,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DeathAfterDelayDeser {
    delay_ms: u64,
}

impl From<DeathAfterDelayDeser> for DeathAfterDelay {
    fn from(deser: DeathAfterDelayDeser) -> Self {
        Self {
            timer: Timer::new(
                Some(Time::from(Duration::from_millis(deser.delay_ms))),
                None,
            ),
        }
    }
}
