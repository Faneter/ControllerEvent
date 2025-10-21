mod controller;
mod event;
mod file;
use controller::{GamepadState, KeyState};
use enigo::{Coordinate, Enigo, Mouse, Settings};
use event::Binder;
use gilrs::{EventType, Gilrs};
use std::time::{Duration, Instant};

use crate::event::{Event, Input};

const MOUSE_SPEED: f32 = 3.0; // 基础鼠标移动速度
const ACCELERATION_FACTOR: f32 = 0.1; // 加速因子
const DEADZONE: f32 = 0.1; // 摇杆死区范围
const MAX_SPEED: f32 = 10.0; // 最大移动速度
const UPDATE_RATE_HZ: u32 = 120; // 更新频率 (Hz)

static mut VEL_X: f32 = 0.0;
static mut VEL_Y: f32 = 0.0;
static mut NOW_X: f32 = 0.0;
static mut NOW_Y: f32 = 0.0;
static mut LAST_X: f32 = 0.0;
static mut LAST_Y: f32 = 0.0;
static mut DELTA_TIME: f32 = 1.0 / UPDATE_RATE_HZ as f32; // 每次更新的时间间隔 (秒)

fn main() {
    #[cfg(target_os = "windows")]
    // This is needed on Windows if you want the application to respect the users scaling settings.
    // Please look at the documentation of the function to see better ways to achive this and
    // important gotchas
    enigo::set_dpi_awareness().unwrap();

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let screen = enigo.main_display().unwrap();
    let mut gilrs = Gilrs::new().unwrap();

    let mut gamepad_state = GamepadState::new();
    let mut binder = Binder::new();

    init_mouse_move_event(&mut binder);

    binder.add_mapping(
        Input::ButtonReleased(gilrs::Button::Select),
        Event::Other(|| {
            println!("Hello, world!");
            return Event::None;
        }),
    );
    binder.add_mapping(
        Input::ButtonPressed(gilrs::Button::West),
        Event::MousePress(enigo::Button::Left),
    );
    binder.add_mapping(
        Input::ButtonReleased(gilrs::Button::West),
        Event::MouseRelease(enigo::Button::Left),
    );
    binder.add_mapping(
        Input::TriggerChanged(gilrs::Button::LeftTrigger2),
        Event::Condition(
            |v1, v2| {
                println!("Trigger value changed from {:?} to {:?}", v2, v1);
                return true;
            },
            Box::new(Event::None),
        ),
    );

    let frame_duration = Duration::from_secs_f32(1.0 / UPDATE_RATE_HZ as f32);
    let mut last_update = Instant::now();

    let exit_requested = false;

    while !exit_requested {
        let now = Instant::now();
        let elapsed = now.duration_since(last_update);

        // 按固定时间步长更新
        if elapsed >= frame_duration {
            unsafe { DELTA_TIME = elapsed.as_secs_f32() };
            last_update = now;
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
                        gamepad_state.update_last_button(
                            gilrs::Button::LeftTrigger2,
                            KeyState::Trigger(value),
                        );
                    }
                    EventType::ButtonChanged(gilrs::Button::RightTrigger2, value, _) => {
                        gamepad_state
                            .update_button(gilrs::Button::RightTrigger2, KeyState::Trigger(value));
                        binder.handle_events(
                            &gamepad_state,
                            &mut enigo,
                            &event::Input::TriggerChanged(gilrs::Button::RightTrigger2),
                        );
                        gamepad_state.update_last_button(
                            gilrs::Button::RightTrigger2,
                            KeyState::Trigger(value),
                        );
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        gamepad_state.update_axis(axis, KeyState::Axis(value));
                        binder.handle_events(
                            &gamepad_state,
                            &mut enigo,
                            &&event::Input::AxisChanged(axis),
                        );
                        gamepad_state.update_last_axis(axis, KeyState::Axis(value));
                    }
                    _ => (),
                }
            }
        }

        calculate_velocity();
        move_mouse(&mut enigo, screen);
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn init_mouse_move_event(binder: &mut Binder) {
    binder.add_mapping(
        Input::AxisChanged(gilrs::Axis::LeftStickX),
        Event::Condition(
            |now, last| {
                if let KeyState::Axis(now_val) = now {
                    if let KeyState::Axis(last_val) = last {
                        unsafe {
                            NOW_X = now_val;
                            LAST_X = last_val;
                        }
                    }
                }
                return false;
            },
            Box::new(Event::None),
        ),
    );
    binder.add_mapping(
        Input::AxisChanged(gilrs::Axis::LeftStickY),
        Event::Condition(
            |now, last| {
                if let KeyState::Axis(now_val) = now {
                    if let KeyState::Axis(last_val) = last {
                        unsafe {
                            NOW_Y = now_val;
                            LAST_Y = last_val;
                        }
                    }
                }
                return false;
            },
            Box::new(Event::None),
        ),
    );
}

fn calculate_velocity() {
    unsafe {
        // x轴
        // 计算当前帧的位移变化
        let delta = NOW_X - LAST_X;
        // 应用加速度
        let acceleration = ACCELERATION_FACTOR * DELTA_TIME;
        // 更新速度
        if delta.abs() > 0.001 {
            // 有输入变化时，直接跟随输入
            VEL_X = NOW_X;
        } else if NOW_X.abs() > DEADZONE {
            // 保持状态时，平滑过渡到目标值
            VEL_X += (NOW_X - VEL_X) * acceleration;
        } else {
            // 在死区内时，逐渐减速
            VEL_X *= 1.0 - (acceleration * 2.0);
        }
        // 限制最大速度
        VEL_X = VEL_X.clamp(-1.0, 1.0);

        // y轴，同上
        let delta = NOW_Y - LAST_Y;
        let acceleration = ACCELERATION_FACTOR * DELTA_TIME;
        if delta.abs() > 0.001 {
            VEL_Y = NOW_Y;
        } else if NOW_Y.abs() > DEADZONE {
            VEL_Y += (NOW_Y - VEL_Y) * acceleration;
        } else {
            VEL_Y *= 1.0 - (acceleration * 2.0);
        }
        VEL_Y = VEL_Y.clamp(-1.0, 1.0);
    }
}

fn move_mouse(enigo: &mut Enigo, screen: (i32, i32)) {
    unsafe {
        // 计算带加速的移动距离
        let speed_factor = MOUSE_SPEED * (1.0 + VEL_X.abs().max(VEL_Y.abs()) * ACCELERATION_FACTOR);
        let mut delta_x = (VEL_X * speed_factor).clamp(-MAX_SPEED, MAX_SPEED);
        let mut delta_y = -(VEL_Y * speed_factor).clamp(-MAX_SPEED, MAX_SPEED); // 反转Y轴

        if delta_x.abs() <= 0.01 && delta_y.abs() <= 0.01 {
            return; // 无需移动
        }
        // 移动鼠标
        enigo
            .move_mouse(delta_x as i32, delta_y as i32, Coordinate::Rel)
            .expect("Failed to move mouse.");
    }
}
