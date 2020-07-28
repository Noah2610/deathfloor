//! Runtime data for events.
//! Some events need to store data, only for their event instance.
//! This data is stored in the `EventsRegister`, and is cleared
//! when switching variants.

use climer::Timer;

pub struct DelayData {
    pub timer: Timer,
}
