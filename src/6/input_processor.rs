use lib::input_parser;

pub type InputType = String;

pub fn get_input(file: &str) -> InputType {
    input_parser::read_file_to_string(file!(), file)
}
