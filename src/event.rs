use crate::controller::{GamepadState, KeyState};
use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse};
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Input {
    ButtonPressed(gilrs::Button),
    ButtonReleased(gilrs::Button),
    TriggerPressed(gilrs::Button),
    TriggerReleased(gilrs::Button),
    TriggerChanged(gilrs::Button),
    AxisChanged(gilrs::Axis),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Event {
    KeyClick(Key),          // 按下并释放单个按键
    KeyPress(Key),          // 按住按键
    KeyRelease(Key),        // 释放按键
    MouseClick(Button),     // 鼠标点击
    MousePress(Button),     // 鼠标按下
    MouseRelease(Button),   // 鼠标释放
    MouseMoveRel(i32, i32), // 相对移动鼠标
    MouseMoveAbs(i32, i32), // 绝对移动鼠标
    MouseScroll(i32, i32),  // 滚动鼠标滚轮
    Macro(Vec<Event>),      // 宏动作序列
    Condition(
        fn(key_state: KeyState, last_key_state: KeyState) -> bool,
        Box<Event>,
    ),
    Other(fn() -> Event), // 其他自定义事件
    None,                 // 无动作
}

pub struct Binder {
    // 输入到输出的映射
    mappings: HashMap<Input, Event>,
    // 组合键映射 (主键 + 修改键) -> 动作
    combo_mappings: HashMap<(Input, Input), Event>,
}

impl Binder {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            combo_mappings: HashMap::new(),
        }
    }

    pub fn add_mapping(&mut self, input: Input, event: Event) {
        self.mappings.insert(input, event);
    }

    pub fn handle_events(&self, gamepad_state: &GamepadState, enigo: &mut Enigo, input: &Input) {
        // 检查组合键
        for ((main, modifier), event) in &self.combo_mappings {
            if input == modifier {
                if Binder::is_active(gamepad_state, main) != None
                    && Binder::is_active(gamepad_state, modifier) != None
                {
                    Binder::excute_event(enigo, event, None);
                    return;
                }
            }
        }

        // 处理单键事件
        if let Some(event) = self.mappings.get(input) {
            if let Some(states) = Binder::is_active(gamepad_state, input) {
                Binder::excute_event(enigo, event, Some(states));
                return;
            }
        }
    }

    pub fn excute_event(enigo: &mut Enigo, event: &Event, states: Option<(KeyState, KeyState)>) {
        match event {
            Event::KeyClick(key) => enigo.key(*key, Direction::Click).unwrap(),
            Event::KeyPress(key) => enigo.key(*key, Direction::Press).unwrap(),
            Event::KeyRelease(key) => enigo.key(*key, Direction::Release).unwrap(),
            Event::MouseClick(button) => enigo.button(*button, Direction::Click).unwrap(),
            Event::MousePress(button) => enigo.button(*button, Direction::Press).unwrap(),
            Event::MouseRelease(button) => enigo.button(*button, Direction::Release).unwrap(),
            Event::MouseMoveRel(x, y) => enigo.move_mouse(*x, *y, Coordinate::Rel).unwrap(),
            Event::MouseMoveAbs(x, y) => enigo.move_mouse(*x, *y, Coordinate::Abs).unwrap(),
            Event::MouseScroll(x, y) => {
                enigo.scroll(*x, enigo::Axis::Horizontal).unwrap();
                enigo.scroll(*y, enigo::Axis::Vertical).unwrap();
            }
            Event::Macro(events) => {
                for event in events {
                    Binder::excute_event(enigo, event, None);
                }
            }
            Event::Condition(cond, boxed_event) => {
                if let Some((key_state, last_key_state)) = states {
                    if cond(key_state, last_key_state) {
                        Binder::excute_event(enigo, boxed_event, None);
                    }
                }
            }
            Event::Other(func) => Binder::excute_event(enigo, &func(), None),
            Event::None => (),
        }
    }

    fn is_active(gamepad_state: &GamepadState, input: &Input) -> Option<(KeyState, KeyState)> {
        match input {
            Input::ButtonPressed(button) => {
                if let Some(KeyState::Key(true)) = gamepad_state.get_button_state(button) {
                    return Some((KeyState::Key(true), KeyState::Key(false)));
                }
            }
            Input::ButtonReleased(button) => {
                if let Some(KeyState::Key(false)) = gamepad_state.get_button_state(button) {
                    return Some((KeyState::Key(false), KeyState::Key(true)));
                }
            }
            Input::TriggerPressed(button) => {
                if let Some(KeyState::Trigger(value)) = gamepad_state.get_button_state(button) {
                    if let Some(KeyState::Trigger(last_value)) =
                        gamepad_state.get_last_button_state(button)
                    {
                        if *value > 0.5 && *last_value <= 0.5 {
                            return Some((
                                KeyState::Trigger(*value),
                                KeyState::Trigger(*last_value),
                            ));
                        }
                    }
                }
            }
            Input::TriggerReleased(button) => {
                if let Some(KeyState::Trigger(value)) = gamepad_state.get_button_state(button) {
                    if let Some(KeyState::Trigger(last_value)) =
                        gamepad_state.get_last_button_state(button)
                    {
                        if *value <= 0.5 && *last_value > 0.5 {
                            return Some((
                                KeyState::Trigger(*value),
                                KeyState::Trigger(*last_value),
                            ));
                        }
                    }
                }
            }
            Input::TriggerChanged(button) => {
                if let Some(KeyState::Trigger(value)) = gamepad_state.get_button_state(button) {
                    if let Some(KeyState::Trigger(last_value)) =
                        gamepad_state.get_last_button_state(button)
                    {
                        if value != last_value {
                            return Some((
                                KeyState::Trigger(*value),
                                KeyState::Trigger(*last_value),
                            ));
                        }
                    }
                }
            }
            Input::AxisChanged(axis) => {
                if let Some(KeyState::Axis(value)) = gamepad_state.get_axis_state(axis) {
                    if let Some(KeyState::Axis(last_value)) =
                        gamepad_state.get_last_axis_state(axis)
                    {
                        if value != last_value {
                            return Some((KeyState::Axis(*value), KeyState::Axis(*last_value)));
                        }
                    }
                }
            }
        }
        return None;
    }
}
