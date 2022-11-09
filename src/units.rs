use std::{time::Duration, i32};

use crate::{frame::{Frame, Drawable}, NUM_ROWS, NUM_COLS};
use rusty_time::timer::Timer;

pub enum UnitType{
    Fighter,
    Shooter,
    Cannon,
}

pub struct Unit {
    unit_type: UnitType,
    x: usize,
    hp: i32,
    damage: i32,
    timer: Timer,
}

const UTIME: u64 = 500;
const FHP: i32 = 3;
const SHP: i32 = 3;
const CHP: i32 = 5;
const FDAM: i32 = 1;
const SDAM: i32 = 1;
const CDAM: i32 = 3;
const UFLOOR: usize = NUM_ROWS - 4;

impl Unit {
    pub fn new(t: UnitType) -> Self {
        match t {
            UnitType::Fighter => Self {
                unit_type: UnitType::Fighter,
                x: 1,
                hp: FHP,
                damage: FDAM,
                timer: Timer::from_millis(UTIME),
            },
            UnitType::Shooter => Self {
                unit_type: UnitType::Shooter,
                x: 1,
                hp: SHP,
                damage: SDAM,
                timer: Timer::from_millis(UTIME),
            },
            UnitType::Cannon => Self {
                unit_type: UnitType::Cannon,
                x: 1,
                hp: CHP,
                damage: CDAM,
                timer: Timer::from_millis(UTIME),
            },
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.hp < 0 {
            self.x += 1;
            self.timer.reset();
        }
    }

    pub fn move_unit(&mut self, distance: usize) {
        self.x += distance;
    }

    pub fn unit_location(&self) -> usize {
        self.x
    }

    pub fn take_damage(&mut self, amt: i32) {
        self.hp -= amt;
    }

    pub fn get_hp(&self) -> i32 {
        self.hp
    }

    pub fn dead(&self) -> bool {
        (self.hp < 1 && self.timer.ready) || (self.x > NUM_COLS - 1)
    }

    pub fn to_string(&self) -> String {
        let mut ret_string = self.hp.to_string();
        ret_string.push_str("|");
        let x_string = self.x.to_string();
        ret_string.push_str(&x_string);
        ret_string
    }
}

impl Drawable for Unit {
    fn draw(&self, frame: &mut Frame) {
        match self.unit_type {
            UnitType::Fighter => {
                frame[self.x][UFLOOR] = "F";
            }
            UnitType::Shooter => {
                frame[self.x][UFLOOR] = "S";
            }
            UnitType::Cannon => {
                frame[self.x][UFLOOR] = "C";
            }
            //_ => {frame[self.x][UFLOOR] = "X";}
        }
    }
}