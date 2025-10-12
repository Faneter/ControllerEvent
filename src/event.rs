use enigo::{Button, Key};

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
    OtherEvent,
}
