#![feature(binary_heap_into_iter_sorted)]
mod input_processor;
use input_processor::{InputType, MonkeyGame, Worry};

use std::time::{Duration, Instant};

type OutputType = u64;

fn part1(input: &InputType) -> OutputType {
    let mut game = input_processor::new_game(input);
    game.worry_adjustment = Worry::Factor(3);
    (0..20).for_each(|_| game.play_round());
    game.get_monkey_business()
}

fn part2(input: &InputType) -> OutputType {
    let mut game = input_processor::new_game(input);
    (0..10000).for_each(|_| game.play_round());
    game.get_monkey_business()
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
        assert_eq!(part1(&input), 10605);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 2713310158);
    }
}
