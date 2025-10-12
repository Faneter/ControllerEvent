mod controller;
mod event;
use controller::{Controller, KeyState};

use std::time::Duration;

use gilrs::{EventType, Gilrs};

fn main() {
    // let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut gilrs = Gilrs::new().unwrap();

    let mut controller = Controller::new();

    let exit_requested = false;

    while !exit_requested {
        while let Some(event) = gilrs.next_event() {
            println!("{:?}", event);
            match event.event {
                EventType::ButtonPressed(button, _) => {
                    controller.update_button(button, KeyState::Key(true));
                }
                EventType::ButtonReleased(button, _) => {
                    controller.update_button(button, KeyState::Key(false));
                }
                EventType::ButtonChanged(trigger, value, _) => {
                    controller.update_button(trigger, KeyState::Trigger(value));
                }
                EventType::AxisChanged(axis, value, _) => {
                    controller.update_axis(axis, KeyState::Axis(value));
                }
                _ => (),
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
