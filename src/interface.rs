use crate::{player::Player, frame::Frame};

pub const INTX: i32 = 2;
pub const INTY: i32 = NUM_ROWS - 1;

pub struct Interface {
    p1hp: int32,
    p1en: int32,
    p2hp: int32,
    p2en: int32,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            p1hp = 0,
            p1en = 0,
            p2hp = 0,
            p2en = 0,
        }
    }
}

impl Drawable for Interface {
    fn draw(&self, frame: &mut Frame) {
        pass;
    }
}