mod input_processor;
use input_processor::{InputElementType, InputType};

use std::time::{Duration, Instant};

type OutputType = u32;

fn filter_if<'b, F>(
    input: &'b InputType,
    threshold: &'b OutputType,
    keep: &'b F,
) -> impl Iterator<Item = OutputType> + 'b
where
    F: Fn(&'b OutputType, &'b OutputType) -> bool + 'b,
{
    input
        .dirs
        .iter()
        .filter_map(move |dir: &'b InputElementType| {
            if keep(&dir.total_size, threshold) {
                Some(dir.total_size)
            } else {
                None
            }
        })
}

fn part1(input: &InputType) -> OutputType {
    const THRESHOLD: OutputType = 100000;
    filter_if(input, &THRESHOLD, &std::cmp::PartialOrd::lt).sum()
}

fn part2(input: &InputType) -> OutputType {
    let threshold = 30000000 - (70000000 - input.dirs[1].total_size);
    filter_if(input, &threshold, &std::cmp::PartialOrd::ge)
        .min()
        .unwrap()
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
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 24933642);
    }
}
