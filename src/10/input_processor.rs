use lib::input_parser;

pub type InputType = Vec<CycleState>;

#[derive(Debug)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

impl std::str::FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        match words.next().unwrap() {
            "addx" => {
                let v: i32 = words.next().unwrap().parse().unwrap();
                Ok(Self::Addx(v))
            }
            "noop" => Ok(Self::Noop),
            &_ => panic!("Unrecognized instruction as input."),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CycleState {
    pub value: i32,
    pub start: u32,
}

impl CycleState {
    pub fn initial() -> Self {
        Self { value: 1, start: 1 }
    }

    fn update(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Addx(v) => {
                self.start += 2;
                self.value += v;
            }
            Instruction::Noop => self.start += 1,
        };
    }
}

pub fn get_input(file: &str) -> InputType {
    let raw = input_parser::read_file_to_string(file!(), file);
    lib::input_parser::read_str_to_struct_iter::<Instruction>(&raw)
        .scan(CycleState::initial(), |state, instruction| {
            state.update(instruction);
            Some(*state)
        })
        .collect::<Vec<CycleState>>()

    // if space is not a concern, better to store full cycle sequence of register values
}
