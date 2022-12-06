use lib::input_parser;

use regex::Regex;

pub type Crate = char;
pub type CrateStack = Vec<Crate>;
pub type InputType = String;

#[derive(Debug)]
// #[recap(regex = r#"move (?P<count>\d+) from (?P<from>d+) to (?P<to>\d+)"#)] can help impl FromStr automatically
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub count: usize,
}

impl std::str::FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let count = lib::input_parser::get_re_name::<usize>(&caps, "count");
        let from = lib::input_parser::get_re_name::<usize>(&caps, "from");
        let to = lib::input_parser::get_re_name::<usize>(&caps, "to");
        Ok(Self { from, to, count })
    }
}

pub fn get_input(file: &str) -> InputType {
    input_parser::read_file_to_string(file!(), file)
}

pub fn get_stacks_and_moves_iter<'a: 'b, 'b>(
    input: &'a str,
    stacks: &mut Vec<CrateStack>,
    moves_iter: &mut Option<Box<dyn Iterator<Item = Move> + 'b>>,
) {
    // assume input is well-formed: stack is numbered sequentially from 1
    let mut input_iter = lib::input_parser::group_lines_iter(input);
    let stacks_str = input_iter.next().unwrap();
    *stacks = get_stacks(stacks_str);
    let moves_str = input_iter.next().unwrap();
    *moves_iter = Some(Box::new(
        lib::input_parser::read_str_to_struct_iter::<Move>(moves_str),
    ));
}

fn get_stacks(stacks_str: &str) -> Vec<CrateStack> {
    let mut lines = stacks_str.lines().rev();
    let num_stacks = lines
        .next()
        .unwrap()
        .split_whitespace()
        // .map(|i| i.parse().unwrap())      only if we want to validate here
        .count();
    let mut stacks: Vec<CrateStack> = vec![Vec::new(); num_stacks];
    lines.for_each(|line| {
        line.chars()
            .enumerate()
            .filter_map(|(i, c)| if i % 4 == 1 { Some(c) } else { None }) // assume "[c] " pattern
            .zip(stacks.iter_mut())
            .for_each(|(c, stack)| {
                if c != ' ' {
                    stack.push(c as char);
                }
            });
    });
    stacks
}
