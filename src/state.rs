use std::time::Instant;
use std::sync::{Mutex, Arc};

pub struct State {
    last_touch_event: Mutex<Instant>
}

impl State {
    pub fn new() -> Arc<State> {
        Arc::new(State { last_touch_event: Mutex::new(Instant::now())})
    }

    pub fn register_touch(&self) {
        let mut last_touch_event = self.last_touch_event.lock().unwrap();
        *last_touch_event = Instant::now();
    }

    pub fn get_time_since_last_touch(&self) -> u64 {
        let mut last_touch_event = self.last_touch_event.lock().unwrap();
        last_touch_event.elapsed().as_secs()
    }
}