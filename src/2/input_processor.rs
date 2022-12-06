use lib::input_parser;

struct Round {
    you: Shape,
    me: i32,
}

impl std::str::FromStr for Round {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ").fuse();
        let you = parts.next().unwrap().parse::<Shape>().unwrap();
        let me = parts.next().unwrap().parse::<Shape>().unwrap() as i32;
        Ok(Self { you, me })
    }
}

pub enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl std::str::FromStr for Shape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
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
    let raw = input_parser::read_file_to_string(file!(), file);
    input_parser::read_str_to_struct_iter::<Round>(&raw)
        .for_each(|round| input[round.me as usize][round.you as usize] += 1);
    input
}
