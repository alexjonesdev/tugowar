use std::time::Duration;
use rusty_time::timer::Timer;
use crate::{units::{Unit, UnitType}, frame::{Drawable, Frame}};

pub const MAX_ENERGY: i32 = 1000;
pub const MIN_ENERGY: i32 = 0;
pub const MAX_HP: i32 = 100;
pub const MIN_HP: i32 = 0;

pub struct Player {
    pub hp: i32,
    pub en: i32,
    pub units: Vec<Unit>,
    en_timer: Timer,
    unit_timer: Timer,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hp: MAX_HP,
            en: MIN_ENERGY,
            units: Vec::new(),
            en_timer: Timer::from_millis(100),
            unit_timer: Timer::from_millis(500),
        }
    }

    pub fn spend_energy(&mut self, e: i32) {
        self.en -= e;
        if self.en < MIN_ENERGY {self.en = MIN_ENERGY;}
    }

    pub fn gain_energy(&mut self, e: i32) {
        self.en += e;
        if self.en > MAX_ENERGY {self.en = MAX_ENERGY;}
    }

    pub fn take_damage(&mut self, d: i32) {
        self.hp -= d;
        if self.en < MIN_HP {self.en = MIN_HP;}
    }

    pub fn spawn_unit(&mut self, t: UnitType) {
         self.units.push(Unit::new(t));
    }

    pub fn update(&mut self, delta: Duration) {
        self.en_timer.update(delta);
        if self.en_timer.ready {
            self.en += 10;
            if self.en > MAX_ENERGY {
                self.en = MAX_ENERGY;
            }
            self.en_timer.reset();
        }
        self.unit_timer.update(delta);
        if self.unit_timer.ready {
            for u in self.units.iter_mut() {
                u.move_unit(1);
            }
            self.unit_timer.reset();
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        for unit in self.units.iter() {
            unit.draw(frame);
        }
    }
}