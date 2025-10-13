use std::collections::HashMap;

use enigo::{Button, Key};

pub enum Input {}

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
