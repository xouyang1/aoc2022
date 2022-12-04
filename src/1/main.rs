#![feature(binary_heap_into_iter_sorted)]

mod input_processor;
use input_processor::InputType;

use std::collections::BinaryHeap;
use std::time::Instant;

type OutputType = u32;

fn part1(input: &InputType) -> OutputType {
    *input.iter().max().unwrap()
}

fn part2(input: &InputType) -> OutputType {
    BinaryHeap::from(input.clone())
        .into_iter_sorted()
        .take(3)
        .sum()
}

fn main() {
    let start = Instant::now();
    let input = input_processor::get_input("input.txt");
    println!("Part 1: {} in {:?}", part1(&input), start.elapsed());
    println!("Part 2: {} in {:?}", part2(&input), start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 45000);
    }
}
