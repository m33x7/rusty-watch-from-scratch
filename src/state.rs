use std::time::Instant;
use std::sync::{Mutex, Arc};

pub struct State {
    last_touch_event: Mutex<Instant>,
    battery_mv: Mutex<Option<u16>>
}

impl State {
    pub fn new() -> Arc<State> {
        Arc::new(State { last_touch_event: Mutex::new(Instant::now()), battery_mv: Mutex::new(None)})
    }

    pub fn register_touch(&self) {
        let mut last_touch_event = self.last_touch_event.lock().unwrap();
        *last_touch_event = Instant::now();
    }

    pub fn get_time_since_last_touch(&self) -> u64 {
        let mut last_touch_event = self.last_touch_event.lock().unwrap();
        last_touch_event.elapsed().as_secs()
    }

    pub fn get_battery_mv(&self) -> Option<u16> {
        let battery_mv = self.battery_mv.lock().unwrap();
        *battery_mv
    }

    pub fn set_battery_mv(&self, mv: u16) {
        let mut battery_mv = self.battery_mv.lock().unwrap();
        *battery_mv = Some(mv);
    }
}