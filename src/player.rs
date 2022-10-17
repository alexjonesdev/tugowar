use crate::units::Unit;

pub const MAX_ENERGY: i32 = 1000;
pub const MIN_ENERGY: i32 = 0;
pub const MAX_HP: i32 = 100;
pub const MIN_HP: i32 = 0;

pub struct Player {
    hp: i32,
    en: i32,
    units: Vec<Unit>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hp: MAX_HP,
            en: MIN_ENERGY,
            units: Vec::new(),
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
}