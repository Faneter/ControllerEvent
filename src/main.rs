mod controller;

use std::time::Duration;

use gilrs::{EventType, Gilrs};

fn main() {
    // let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut gilrs = Gilrs::new().unwrap();

    let mut exit_requested = false;

    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    while !exit_requested {
        while let Some(event) = gilrs.next_event() {
            println!("{:?}", event);
            match event.event {
                EventType::ButtonPressed(gilrs::Button::Select, _) => {
                    exit_requested = true;
                }
                _ => println!("123"),
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
