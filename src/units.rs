pub enum UnitType{
    Fighter,
    Shooter,
    Cannon,
}

pub struct Unit {
    pub x: usize,
    pub y: usize,
    hp : i32,
}

const FHP: i32 = 3;
const SHP: i32 = 3;
const CHP: i32 = 3;

impl Unit {
    pub fn new(t: UnitType) -> Self {
        match t {
            UnitType::Fighter => Self {
                x: 1,
                y: 1,
                hp: FHP,
            },

            UnitType::Shooter => Self {
                x: 1,
                y: 1,
                hp: SHP,
            },

            UnitType::Cannon => Self {
                x: 1,
                y: 1,
                hp: CHP,
            },
        }
    }

    pub fn move_unit(&mut self, distance: usize) {
        self.x += distance;
    }
}
