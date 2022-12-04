use lib::input_parser;

use std::fmt::Debug;
use std::iter::Sum;
use std::str::FromStr;

type InputElementType = u32;
pub type InputType = Vec<InputElementType>;

pub fn get_input(file: &str) -> InputType {
    let raw = input_parser::read_file_to_string(file!(), file);
    input_parser::group_lines_iter(&raw)
        .map(|group| process_input_group::<InputElementType>(group))
        .collect()
}

fn process_input_group<T: FromStr + Sum>(group: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    group
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<T>().unwrap())
        .sum()
}
