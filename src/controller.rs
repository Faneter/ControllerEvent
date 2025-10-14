use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum KeyState {
    Key(bool),
    Trigger(f32),
    Axis(f32),
}

pub struct GamepadState {
    buttons: HashMap<gilrs::Button, KeyState>,
    axes: HashMap<gilrs::Axis, KeyState>,
    last_buttons: HashMap<gilrs::Button, KeyState>,
    last_axes: HashMap<gilrs::Axis, KeyState>,
}

impl GamepadState {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
            axes: HashMap::new(),
            last_buttons: HashMap::new(),
            last_axes: HashMap::new(),
        }
    }

    pub fn update_button(&mut self, key: gilrs::Button, state: KeyState) {
        self.buttons.insert(key, state);
    }

    pub fn update_last_button(&mut self, key: gilrs::Button, state: KeyState) {
        self.last_buttons.insert(key, state);
    }

    pub fn update_axis(&mut self, axis: gilrs::Axis, state: KeyState) {
        self.axes.insert(axis, state);
    }

    pub fn update_last_axis(&mut self, axis: gilrs::Axis, state: KeyState) {
        self.last_axes.insert(axis, state);
    }

    pub fn get_button_state(&self, button: &gilrs::Button) -> Option<&KeyState> {
        return self.buttons.get(button);
    }

    pub fn get_last_button_state(&self, button: &gilrs::Button) -> Option<&KeyState> {
        return self.last_buttons.get(button);
    }

    pub fn get_axis_state(&self, axis: &gilrs::Axis) -> Option<&KeyState> {
        return self.axes.get(&axis);
    }

    pub fn get_last_axis_state(&self, axis: &gilrs::Axis) -> Option<&KeyState> {
        return self.last_axes.get(&axis);
    }
}
