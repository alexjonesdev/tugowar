use crate::{frame::{Frame, Drawable}, NUM_ROWS, NUM_COLS};

const INTXP1: usize = 0;
const INTYP1: usize = NUM_ROWS - 2;
const INTXP2: usize = NUM_COLS - 16;
const INTYP2: usize = NUM_ROWS - 2;
const INTLEN: i32 = 10;
const HPSIZE: i32 = 10;
const ENSIZE: i32 = 100;

pub struct Interface {
    p1hp: i32,
    p1en: i32,
    p2hp: i32,
    p2en: i32,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            p1hp: 0,
            p1en: 0,
            p2hp: 0,
            p2en: 0,
        }
    }

    pub fn update(&mut self, p1hp: i32, p1en: i32, p2hp: i32, p2en: i32) {
        self.p1hp = p1hp;
        self.p1en = p1en;
        self.p2hp = p2hp;
        self.p2en = p2en;
    }
}

impl Drawable for Interface {
    fn draw(&self, frame: &mut Frame) {
        //Draw screen border
        for x in 0..NUM_COLS {
            frame[x][0] = "-";
        }

        for y in 1..NUM_ROWS - 3 {
            frame[0][y] = "|";
            frame[NUM_COLS - 1][y] = "|";
        }

        for x in 0..NUM_COLS {
            frame[x][NUM_ROWS - 3] = "-";
        }

        //Draw Player Bars
        let hp_start = "HP: [";
        let en_start = "EN: [";
        frame[INTXP1][INTYP1] = hp_start;
        frame[INTXP1][INTYP1 + 1] = en_start;
        frame[INTXP2][INTYP1] = hp_start;
        frame[INTXP2][INTYP1 + 1] = en_start;

        for x in 0..=INTLEN {
            //Draw P1 HP
            if x <= (self.p1hp / HPSIZE) {
                frame[INTXP1 + hp_start.len() + x as usize][INTYP1] = "|";
            } else {
                frame[INTXP1 + hp_start.len() + x as usize][INTYP1] = "-";
            }

            //Draw P1 EN
            if x < (self.p1en / ENSIZE) {
                frame[INTXP1 + en_start.len() + x as usize][INTYP1 + 1] = "|";
            } else {
                frame[INTXP1 + en_start.len() + x as usize][INTYP1 + 1] = "-";
            }

            //Draw P2 HP
            if x <= (self.p1hp / HPSIZE) {
                frame[INTXP2 + hp_start.len() + x as usize][INTYP2] = "|";
            } else {
                frame[INTXP2 + hp_start.len() + x as usize][INTYP2] = "-";
            }

            //Draw P2 EN
            if x < (self.p1en / ENSIZE) {
                frame[INTXP2 + en_start.len() + x as usize][INTYP2 + 1] = "|";
            } else {
                frame[INTXP2 + en_start.len() + x as usize][INTYP2 + 1] = "-";
            }
        }
        frame[INTXP1 + INTLEN as usize + hp_start.len()][INTYP1] = "]";
        frame[INTXP1 + INTLEN as usize + en_start.len()][INTYP1 + 1] = "]";
        frame[INTXP2 + INTLEN as usize + hp_start.len()][INTYP2] = "]";
        frame[INTXP2 + INTLEN as usize + en_start.len()][INTYP2 + 1] = "]";
        //print!("{}:{}|", self.p1en, self.p1en / 100);
    }
}