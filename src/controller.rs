
use std::collections::HashMap;

pub enum KeyState {
    Key(bool),
    Trigger(f32),
    Axis(f32),
}

pub struct Controller {
    buttons: HashMap<gilrs::Button, KeyState>,
    axies: HashMap<gilrs::Axis, KeyState>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
            axies: HashMap::new(),
        }
    }

    pub fn update_button(&mut self, key: gilrs::Button, state: KeyState) {
        self.buttons.insert(key, state);
    }

    pub fn update_axis(&mut self, axis: gilrs::Axis, state: KeyState) {
        self.axies.insert(axis, state);
    }
}
