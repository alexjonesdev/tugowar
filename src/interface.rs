use crate::{frame::{Frame, Drawable}, NUM_ROWS, NUM_COLS};

const INTXP1: usize = 1;
const INTYP1: usize = NUM_ROWS - 3;
const INTXP2: usize = NUM_COLS - 17;
const INTYP2: usize = NUM_ROWS - 3;
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
        let hpStart = "HP: [";
        let enStart = "EN: [";

        frame[INTXP1][INTYP1] = hpStart;
        frame[INTXP1][INTYP1 + 1] = enStart;
        frame[INTXP2][INTYP1] = hpStart;
        frame[INTXP2][INTYP1 + 1] = enStart;
        for x in 0..=INTLEN {
            //Draw P1 HP
            if x <= (self.p1hp / HPSIZE) {
                frame[INTXP1 + hpStart.len() + x as usize][INTYP1] = "|";
            } else {
                frame[INTXP1 + hpStart.len() + x as usize][INTYP1] = "-";
            }

            //Draw P1 EN
            if x < (self.p1en / ENSIZE) {
                frame[INTXP1 + enStart.len() + x as usize][INTYP1 + 1] = "|";
            } else {
                frame[INTXP1 + enStart.len() + x as usize][INTYP1 + 1] = "-";
            }

            //Draw P2 HP
            if x <= (self.p1hp / HPSIZE) {
                frame[INTXP2 + hpStart.len() + x as usize][INTYP2] = "|";
            } else {
                frame[INTXP2 + hpStart.len() + x as usize][INTYP2] = "-";
            }

            //Draw P2 EN
            if x < (self.p1en / ENSIZE) {
                frame[INTXP2 + enStart.len() + x as usize][INTYP2 + 1] = "|";
            } else {
                frame[INTXP2 + enStart.len() + x as usize][INTYP2 + 1] = "-";
            }
        }
        frame[INTXP1 + INTLEN as usize + hpStart.len()][INTYP1] = "]";
        frame[INTXP1 + INTLEN as usize + enStart.len()][INTYP1 + 1] = "]";
        frame[INTXP2 + INTLEN as usize + hpStart.len()][INTYP2] = "]";
        frame[INTXP2 + INTLEN as usize + enStart.len()][INTYP2 + 1] = "]";
        //print!("{}:{}|", self.p1en, self.p1en / 100);
    }
}