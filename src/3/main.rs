mod input_processor;
use input_processor::InputType;
use lib::input_parser;

use itertools::Itertools;
use std::collections::HashSet;
use std::time::{Duration, Instant};

fn get_item_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        panic!("Expecting only alphabetical characters but found {c}");
    }
}

fn part1(input: &InputType) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let (compartment1, compartment2) = input_parser::str_split_half(rucksack);
            let compartment1: HashSet<char> = compartment1.chars().collect();
            for c in compartment2.chars() {
                if compartment1.contains(&c) {
                    return get_item_priority(c);
                }
            }
            panic!("No match found");
        })
        .sum()
}

fn part2(input: &InputType) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let mut sets = group.map(|rucksack| rucksack.chars().collect::<HashSet<char>>());
            let mut intersection = sets.next().unwrap();
            let sets: Vec<HashSet<char>> = sets.collect();
            // in-place processing instead of std::collections::hash_set::Intersection
            intersection.retain(|c| sets.iter().all(|s| s.contains(c)));
            if intersection.len() > 1 {
                panic!("Multiple matching values {:?}", intersection);
            }
            get_item_priority(*intersection.iter().next().unwrap())
        })
        .sum()
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

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 70);
    }
}
