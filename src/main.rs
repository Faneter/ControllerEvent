mod controller;
mod event;
use controller::{GamepadState, KeyState};
use enigo::{Enigo, Settings};

use std::time::Duration;

use gilrs::{EventType, Gilrs};

use crate::event::Binder;

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut gilrs = Gilrs::new().unwrap();

    let mut gamepad_state = GamepadState::new();
    let mut binder = Binder::new();

    binder.add_mapping(
        event::Input::Button(gilrs::Button::Select),
        event::Event::OtherEvent(Box::new(|| println!("Hello"))),
    );

    let exit_requested = false;

    while !exit_requested {
        while let Some(event) = gilrs.next_event() {
            // println!("{:?}", event);
            match event.event {
                EventType::ButtonPressed(button, _) => {
                    gamepad_state.update_button(button, KeyState::Key(true));
                    binder.handle_events(&gamepad_state, &event::Input::Button(button));
                    gamepad_state.update_last_button(button, KeyState::Key(true));
                }
                EventType::ButtonReleased(button, _) => {
                    gamepad_state.update_button(button, KeyState::Key(false));
                    binder.handle_events(&gamepad_state, &event::Input::Button(button));
                    gamepad_state.update_last_button(button, KeyState::Key(false));
                }
                EventType::ButtonChanged(trigger, value, _) => {
                    gamepad_state.update_button(trigger, KeyState::Trigger(value));
                    gamepad_state.update_last_button(trigger, KeyState::Trigger(value));
                }
                EventType::AxisChanged(axis, value, _) => {
                    gamepad_state.update_axis(axis, KeyState::Axis(value));
                    gamepad_state.update_last_axis(axis, KeyState::Axis(value));
                }
                _ => (),
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
