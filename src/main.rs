mod controller;
mod event;
use controller::{GamepadState, KeyState};
use enigo::{Enigo, Settings};
use event::Binder;
use gilrs::{EventType, Gilrs};
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut gilrs = Gilrs::new().unwrap();

    let mut gamepad_state = GamepadState::new();
    let mut binder = Binder::new();

    binder.add_mapping(
        event::Input::ButtonReleased(gilrs::Button::Select),
        event::Event::Other(|| println!("Hello")),
    );

    let exit_requested = false;

    while !exit_requested {
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::ButtonPressed(button, _) => {
                    gamepad_state.update_button(button, KeyState::Key(true));
                    binder.handle_events(
                        &gamepad_state,
                        &mut enigo,
                        &&event::Input::ButtonPressed(button),
                    );
                    gamepad_state.update_last_button(button, KeyState::Key(true));
                }
                EventType::ButtonReleased(button, _) => {
                    gamepad_state.update_button(button, KeyState::Key(false));
                    binder.handle_events(
                        &gamepad_state,
                        &mut enigo,
                        &&event::Input::ButtonReleased(button),
                    );
                    gamepad_state.update_last_button(button, KeyState::Key(false));
                }
                EventType::ButtonChanged(gilrs::Button::LeftTrigger2, value, _) => {
                    gamepad_state
                        .update_button(gilrs::Button::LeftTrigger2, KeyState::Trigger(value));
                    binder.handle_events(
                        &gamepad_state,
                        &mut enigo,
                        &event::Input::TriggerChanged(gilrs::Button::LeftTrigger2),
                    );
                    gamepad_state
                        .update_last_button(gilrs::Button::LeftTrigger2, KeyState::Trigger(value));
                }
                EventType::ButtonChanged(gilrs::Button::RightTrigger2, value, _) => {
                    gamepad_state
                        .update_button(gilrs::Button::RightTrigger2, KeyState::Trigger(value));
                    gamepad_state
                        .update_last_button(gilrs::Button::RightTrigger2, KeyState::Trigger(value));
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

fn init_mouse_move_event() {}
