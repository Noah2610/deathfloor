use super::system_prelude::*;
use deathframe::amethyst::utils::fps_counter::FpsCounter;
use std::time::{Duration, Instant};

pub struct DebugSystem {
    print_delay: Duration,
    to_print:    Vec<String>,
    last_print:  Instant,
}

impl<'a> System<'a> for DebugSystem {
    type SystemData = Read<'a, FpsCounter>;

    fn run(&mut self, fps_counter: Self::SystemData) {
        let now = Instant::now();

        if now - self.last_print >= self.print_delay {
            self.print_fps(&fps_counter);
            self.last_print = now;
        }

        self.print();
    }
}

impl DebugSystem {
    /// Returns a new `DebugSystem` with the given print delay in milliseconds.
    pub fn new(print_delay_ms: u64) -> Self {
        Self {
            print_delay: Duration::from_millis(print_delay_ms),
            to_print:    Vec::new(),
            last_print:  Instant::now(),
        }
    }

    fn print(&mut self) {
        if !self.to_print.is_empty() {
            println!("{}", self.to_print.join("\n"));
        }
        self.to_print.clear();
    }

    fn print_fps(&mut self, fps_counter: &FpsCounter) {
        let fps_frame = fps_counter.frame_fps();
        let fps_avg = fps_counter.sampled_fps();
        self.push_text(format!(
            "fps: {:.02} (avg: {:.02})",
            fps_frame, fps_avg
        ));
    }

    fn push_text<S>(&mut self, text: S)
    where
        S: ToString,
    {
        self.to_print.push(text.to_string());
    }
}

impl Default for DebugSystem {
    fn default() -> Self {
        Self {
            print_delay: Duration::from_secs(1),
            to_print:    Vec::new(),
            last_print:  Instant::now(),
        }
    }
}
