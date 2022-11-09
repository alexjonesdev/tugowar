use std::time::Duration;
use rusty_time::timer::Timer;
use crate::{units::{Unit, UnitType}, frame::{Drawable, Frame}, NUM_COLS};

pub const MAX_ENERGY: i32 = 1000;
pub const MIN_ENERGY: i32 = 0;
pub const MAX_HP: i32 = 100;
pub const MIN_HP: i32 = 0;

pub struct Player {
    pub hp: i32,
    pub en: i32,
    pub units: Vec<Unit>,
    en_timer: Timer,
    pub dd: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hp: MAX_HP,
            en: MIN_ENERGY,
            units: Vec::new(),
            en_timer: Timer::from_millis(100),
            dd: 0,
        }
    }

    fn spend_energy(&mut self, e: i32) -> bool {
        if self.en - e < MIN_ENERGY {
            false
        } else {
            self.en -= e;
            true
        }
    }

    pub fn gain_energy(&mut self, e: i32) {
        self.en += e;
        if self.en > MAX_ENERGY {self.en = MAX_ENERGY;}
    }

    pub fn take_damage(&mut self, d: i32) {
        self.hp -= d;
        if self.en < MIN_HP {self.en = MIN_HP;}
    }

    pub fn get_dd(&self) -> i32 {
        self.dd
    }

    pub fn reset_dd(&mut self) {
        self.dd = 0;
    }

    pub fn spawn_unit(&mut self, t: UnitType) {
        match t {
            UnitType::Fighter => {
                if self.spend_energy(100) {
                    self.units.push(Unit::new(t));
                }
            }
            UnitType::Shooter => {
                if self.spend_energy(100) {
                    self.units.push(Unit::new(t));
                }
            }
            UnitType::Cannon => {
                if self.spend_energy(300) {
                    self.units.push(Unit::new(t));
                }
            }
        }
        //println!("{}",self.units.len());
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
        for unit in self.units.iter_mut() {
            unit.update(delta);

            if unit.unit_location() > NUM_COLS - 1 {
                self.dd += unit.get_hp();
            }
        }
        self.units.retain(|unit| !unit.dead());
    }

    pub fn print_units(&self) {
        let mut prnt_string: String = String::new();
        for unit in self.units.iter() {
            let u_string = unit.to_string();
            prnt_string.push_str(&u_string);
            prnt_string.push_str(",");
        }
        println!("{}",prnt_string);
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        for unit in self.units.iter() {
            unit.draw(frame);
        }
    }
}