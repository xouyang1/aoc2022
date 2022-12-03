use lib::input_parser;

pub type InputType = String;

pub fn get_input(file: &str) -> InputType {
    // mem::variant_count::<Shape> unstable (https://github.com/rust-lang/rust/issues/73662) so hardcoding to 3 for now
    input_parser::read_file_to_string(file!(), file)
}
