mod input_processor;
use input_processor::InputType;

use std::time::{Duration, Instant};

type OutputType = u32;

fn part1(input: &InputType) -> OutputType {
    0
}

fn part2(input: &InputType) -> OutputType {
    0
}

fn main() {
    let start = Instant::now();
    let input = input_processor::get_input("input.txt");
    let input_duration =
        lib::display::print_results("Input Processing", "", Duration::new(0, 0), start);
    let part1_duration =
        lib::display::print_results("Part 1", part1(&input), input_duration, start);
    lib::display::print_results("Part 2", part2(&input), part1_duration, start);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part1(&input), 157);
    }

    // #[test]
    // fn part2_test() {
    //     let input = input_processor::get_input("input-test.txt");
    //     assert_eq!(part2(&input), 70);
    // }
}
