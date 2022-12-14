mod input_processor;
use input_processor::InputType;

use std::time::{Duration, Instant};

use itertools::Itertools;

type InputElementType = u32;
type OutputType = u32;
type Assignment = std::ops::Range<InputElementType>;
type PairAssignments = (Assignment, Assignment);

fn count_if<F>(input: &InputType, keep: F) -> usize
where
    F: Fn(&PairAssignments) -> bool,
{
    input
        .lines()
        .filter(|&line| {
            let pair = line
                .split(&['-', ','][..])
                .map(|i| i.parse().unwrap())
                .tuples::<(InputElementType, InputElementType)>()
                .map(|tup| tup.0..tup.1)
                .collect_tuple::<(Assignment, Assignment)>()
                .unwrap();
            keep(&pair)
        })
        .count()
}

fn part1(input: &InputType) -> OutputType {
    let keep_subset_pair = |pair: &PairAssignments| -> bool {
        (pair.0.start as i32 - pair.1.start as i32) * (pair.1.end as i32 - pair.0.end as i32) >= 0
    };
    count_if(input, keep_subset_pair) as OutputType
}

fn part2(input: &InputType) -> OutputType {
    let keep_overlapping_pair = |pair: &PairAssignments| -> bool {
        !(pair.0.end < pair.1.start || pair.1.end < pair.0.start)
    };
    count_if(input, keep_overlapping_pair) as OutputType
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
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 4);
    }
}
