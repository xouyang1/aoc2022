mod input_processor;
use input_processor::{CrateStack, InputType, Move};

use std::time::{Duration, Instant};

type OutputType = String;

fn rearrange_one_crate_per_op(stacks: &mut Vec<CrateStack>, moves: impl Iterator<Item = Move>) {
    moves.for_each(|update| {
        (1..=update.count).for_each(|_| {
            let c = stacks[update.from - 1].pop().unwrap();
            stacks[update.to - 1].push(c);
        });
    });
}

fn rearrange_all_crates_per_op(stacks: &mut Vec<CrateStack>, moves: impl Iterator<Item = Move>) {
    moves.for_each(|update| {
        let from_stack = &mut stacks[update.from - 1];
        let group = from_stack.split_off(from_stack.len() - update.count);
        stacks[update.to - 1].extend(group);
    });
}

fn get_top(stacks: &Vec<CrateStack>) -> String {
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part1<'a: 'b, 'b>(input: &'a InputType) -> OutputType {
    let mut stacks = Vec::new();
    let mut moves: Option<Box<dyn Iterator<Item = Move> + 'b>> = None;
    input_processor::get_stacks_and_moves_iter(input, &mut stacks, &mut moves);
    rearrange_one_crate_per_op(&mut stacks, moves.unwrap());
    get_top(&stacks)
}

fn part2<'a: 'b, 'b>(input: &'a InputType) -> OutputType {
    let mut stacks = Vec::new();
    let mut moves: Option<Box<dyn Iterator<Item = Move> + 'b>> = None;
    input_processor::get_stacks_and_moves_iter(input, &mut stacks, &mut moves);
    rearrange_all_crates_per_op(&mut stacks, moves.unwrap());
    get_top(&stacks)
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
        assert_eq!(part1(&input), "CMZ");
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), "MCD");
    }
}
