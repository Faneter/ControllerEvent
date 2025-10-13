mod controller;
mod event;
use controller::{GamepadState, KeyState};
use enigo::{Enigo, Settings};

use std::time::Duration;

use gilrs::{EventType, Gilrs};

fn main() {
    // let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut gilrs = Gilrs::new().unwrap();

    let mut gamepad_state = GamepadState::new();

    let exit_requested = false;

    let mut last_event: Option<EventType> = None;

    while !exit_requested {
        while let Some(event) = gilrs.next_event() {
            println!("{:?}", event);
            last_event = Some(event.event);
            match event.event {
                EventType::ButtonPressed(button, _) => {
                    gamepad_state.update_button(button, KeyState::Key(true));
                }
                EventType::ButtonReleased(button, _) => {
                    gamepad_state.update_button(button, KeyState::Key(false));
                }
                EventType::ButtonChanged(trigger, value, _) => {
                    gamepad_state.update_button(trigger, KeyState::Trigger(value));
                }
                EventType::AxisChanged(axis, value, _) => {
                    gamepad_state.update_axis(axis, KeyState::Axis(value));
                }
                _ => (),
            }
        }

        // 事件处理

        // 事件处理结束
        if let Some(event) = last_event {
            match event {
                EventType::ButtonPressed(button, _) => {
                    gamepad_state.update_last_button(button, KeyState::Key(true));
                }
                EventType::ButtonReleased(button, _) => {
                    gamepad_state.update_last_button(button, KeyState::Key(false));
                }
                EventType::ButtonChanged(trigger, value, _) => {
                    gamepad_state.update_last_button(trigger, KeyState::Trigger(value));
                }
                EventType::AxisChanged(axis, value, _) => {
                    gamepad_state.update_last_axis(axis, KeyState::Axis(value));
                }
                _ => (),
            }
        }

        std::thread::sleep(Duration::from_millis(1));
    }
}
