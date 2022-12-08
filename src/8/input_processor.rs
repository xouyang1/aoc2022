use lib::input_parser;

pub type InputType = Vec<Vec<u32>>;

pub fn get_input(file: &str) -> InputType {
    input_parser::read_file_to_string(file!(), file)
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}
