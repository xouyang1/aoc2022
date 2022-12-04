use lib::input_parser;

use std::fmt::Debug;

use num_derive::FromPrimitive;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
struct Round {
    you: Shape,
    me: i32,
}

impl<'de> Deserialize<'de> for Round {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer).unwrap();
        let mut parts = s.split(" ").fuse();
        let you = parts.next().unwrap().into();
        let me: Shape = parts.next().unwrap().into();
        let me: i32 = me as i32;
        Ok(Round { you, me })
    }
}

// FromPrimitive::from_i32
#[derive(Clone, Copy, Debug, FromPrimitive, Hash, PartialEq, Eq, PartialOrd)]
pub enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl From<&str> for Shape {
    fn from(item: &str) -> Self {
        match item {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            &_ => panic!("Unrecognized shape."),
        }
    }
}

pub enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

// mem::variant_count::<Shape> unstable (https://github.com/rust-lang/rust/issues/73662) so hardcoding to 3 for now
pub type InputSubType = [i32; 3];
pub type InputType = [InputSubType; 3];

pub fn get_input(file: &str) -> InputType {
    let mut input = InputType::default();
    input_parser::read_file_to_struct::<Round>(file!(), file)
        .iter()
        .for_each(|round| input[round.me as usize][round.you as usize] += 1);
    input
}
