use std::collections::HashMap;

use enigo::{Button, Key};
use gilrs::ev;

use crate::{
    controller::{GamepadState, KeyState},
    event,
};

#[derive(Eq, Hash, PartialEq)]
pub enum Input {
    Button(gilrs::Button),
}

pub enum Event {
    KeyClick(Key),         // 按下并释放单个按键
    KeyPress(Key),         // 按住按键
    KeyRelease(Key),       // 释放按键
    MouseClick(Button),    // 鼠标点击
    MousePress(Button),    // 鼠标按下
    MouseRelease(Button),  // 鼠标释放
    MouseMove(i32, i32),   // 相对移动鼠标
    MouseScroll(i32, i32), // 滚动鼠标滚轮
    Macro(Vec<Event>),     // 宏动作序列
    OtherEvent(Box<dyn Fn()>),
}

pub struct Binder {
    // 输入到输出的映射
    mappings: HashMap<Input, Event>,
    // 组合键映射 (主键 + 修改键) -> 动作
    combo_mappings: HashMap<(Input, Input), Event>,
    // 特殊模式配置
    toggle_modes: HashMap<Input, (Event, Event)>, // 切换模式
    hold_modes: HashMap<Input, (Event, Event)>,   // 按住模式
}

impl Binder {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            combo_mappings: HashMap::new(),
            toggle_modes: HashMap::new(),
            hold_modes: HashMap::new(),
        }
    }

    pub fn add_mapping(&mut self, input: Input, event: Event) {
        self.mappings.insert(input, event);
    }

    pub fn handle_events(&self, gamepad_state: &GamepadState, input: &Input) {
        if let Some(event) = self.mappings.get(input) {
            match input {
                Input::Button(button) => {
                    if let Some(state) = gamepad_state.get_button_state(button) {
                        if let KeyState::Key(true) = state {
                            if let Event::OtherEvent(func) = event {
                                func();
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
