use crate::frame::{Frame, Drawable};

pub enum UnitType{
    Fighter,
    Shooter,
    Cannon,
}

pub struct Unit {
    unit_type: UnitType,
    pub x: usize,
    pub y: usize,
    hp: i32,
    damage: i32,
}

const FHP: i32 = 3;
const SHP: i32 = 3;
const CHP: i32 = 3;
const FDAM: i32 = 1;
const SDAM: i32 = 1;
const CDAM: i32 = 3;

impl Unit {
    pub fn new(t: UnitType) -> Self {
        match t {
            UnitType::Fighter => Self {
                unit_type: UnitType::Fighter,
                x: 1,
                y: 1,
                hp: FHP,
                damage: FDAM,
            },

            UnitType::Shooter => Self {
                unit_type: UnitType::Shooter,
                x: 1,
                y: 1,
                hp: SHP,
                damage: SDAM,
            },

            UnitType::Cannon => Self {
                unit_type: UnitType::Cannon,
                x: 1,
                y: 1,
                hp: CHP,
                damage: CDAM,
            },
        }
    }

    pub fn move_unit(&mut self, distance: usize) {
        self.x += distance;
    }
}

impl Drawable for Unit {
    fn draw(&self, frame: &mut Frame) {
        match self.unit_type {
            UnitType::Fighter => {
                frame[self.x][self.y] = "F";
            }
            UnitType::Shooter => {
                frame[self.x][self.y] = "S";
            }
            UnitType::Cannon => {
                frame[self.x][self.y] = "C";
            }
            _ => {}
        }
    }
}