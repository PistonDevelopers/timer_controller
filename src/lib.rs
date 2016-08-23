#![deny(missing_docs)]

//! A timer controller.

extern crate input;

use input::GenericEvent;

/// A timer relative to start of program.
pub struct Timer {
    /// The interval in seconds between each trigger.
    pub interval: f64,
    /// The time in seconds from start of program.
    pub time: f64,
    /// The time of next trigger in seconds.
    pub next: f64,
}

impl Timer {
    /// Creates a new timer.
    pub fn new(interval: f64) -> Timer {
        Timer {
            interval: interval,
            time: 0.0,
            next: 0.0,
        }
    }

    /// Calls closure for each interval to catch up with update time.
    ///
    /// The timing is inaccurate for less intervals than the update interval.
    pub fn event<E: GenericEvent, F: FnMut()>(&mut self, e: &E, mut f: F) {
        if let Some(args) = e.update_args() {
            self.time = self.time + args.dt;
            while self.next <= self.time {
                self.next = self.next + self.interval;
                f();
            }
        }
    }
}
