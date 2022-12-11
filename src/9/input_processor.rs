use lib::input_parser;
use std::str::FromStr;

pub type InputType = Vec<Motion>;
pub type Position = (i32, i32);

pub fn get_input(file: &str) -> InputType {
    input_parser::read_file_to_string(file!(), file)
        .lines()
        .map(|line| line.parse::<Motion>().unwrap())
        .collect()
}

pub struct Motion {
    pub dir: Position,
    pub count: u32,
}

impl FromStr for Motion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s.chars().next().unwrap() {
            'R' => (1, 0),
            'L' => (-1, 0),
            'D' => (0, 1),
            'U' => (0, -1),
            _ => panic!("Unrecognized input direction"),
        };
        let count: u32 = s[2..].parse().unwrap();
        Ok(Self { dir, count })
    }
}
