use log::{info, warn};
use std::fmt;

use super::dice;

pub struct Attributes {
    scores: [u8; 6],
}

impl Attributes {
    fn new(scores: [u8; 6]) -> Self {
        Attributes { scores }
    }

    /// Generates a new, random set of attributes and (TODO) replaces the lowest value with 14.
    pub fn new_random() -> Self {
        let mut new_scores = [0_u8; 6];

        info!("rolling scores...");
        for score in new_scores.iter_mut() {
            *score = dice::d6() + dice::d6() + dice::d6();
        }

        info!("replacing lowest score(s) with 14");
        let min = new_scores.iter().min().unwrap();
        let mindex = new_scores
            .iter()
            .position(|element| element == min)
            .unwrap();
        println!("{mindex}");

        new_scores[mindex] = 14;

        Self::new(new_scores)
    }

    /// Returns a reference to the score values in str-dex-con-int-wis-cha order.
    pub fn get_scores(&self) -> &[u8; 6] {
        &self.scores
    }

    /// Returns a byte-array of signed u8s from -2 to -2 in str-dex-con-int-wis-cha order.
    pub fn get_mods(&self) -> [i8; 6] {
        let mut out_array = [0_i8; 6];
        for (i, score) in self.get_scores().into_iter().enumerate() {
            out_array[i] = match score {
                0..=3 => -2,
                4..=7 => -1,
                14..=17 => 1,
                18 => 2,
                _ => 0,
            }
        }

        out_array
    }
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ts = self.get_scores();
        let ms = self.get_mods();

        write!(
            f,
            "str: {:>2} ({:+>2})\ndex: {:>2} ({:+>2})\ncon: {:>2} ({:+>2})\nint: {:>2} ({:+>2})\nwis: {:>2} ({:+>2})\ncha: {:>2} ({:+>2})",
            ts[0], ms[0],
            ts[1], ms[1],
            ts[2], ms[2],
            ts[3], ms[3],
            ts[4], ms[4],
            ts[5], ms[5],
        )
    }
}
