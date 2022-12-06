mod input_processor;
use input_processor::InputType;

use std::collections::HashMap;
use std::time::{Duration, Instant};

type OutputType = usize;

fn remove_and_update_dups(
    counts_map: &mut HashMap<char, usize>,
    num_dups: &mut OutputType,
    c: char,
) {
    counts_map.entry(c).and_modify(|e| {
        *e -= 1;
        if *e == 1 {
            *num_dups -= 1;
        }
    });
}

fn add_and_update_dups(counts_map: &mut HashMap<char, usize>, num_dups: &mut OutputType, c: char) {
    counts_map
        .entry(c)
        .and_modify(|e| {
            *e += 1;
            if *e == 2 {
                *num_dups += 1;
            }
        })
        .or_insert(1);
}

fn get_marker(input: &InputType, window_size: usize) -> OutputType {
    let mut counts_map = HashMap::new();
    let mut num_dups: OutputType = 0;
    input[0..window_size].chars().for_each(|c| {
        add_and_update_dups(&mut counts_map, &mut num_dups, c);
    });
    input[window_size..]
        .char_indices()
        .zip(input.chars())
        .filter_map(|((i, next), prev)| {
            add_and_update_dups(&mut counts_map, &mut num_dups, next);
            remove_and_update_dups(&mut counts_map, &mut num_dups, prev);
            match num_dups {
                0 => Some(i + 1 + window_size),
                _ => None,
            }
        })
        .next()
        .unwrap()
}

fn part1(input: &InputType) -> OutputType {
    get_marker(input, 4)
}

fn part2(input: &InputType) -> OutputType {
    get_marker(input, 14)
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
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 19);
    }
}
