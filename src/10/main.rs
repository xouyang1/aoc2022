mod input_processor;
use input_processor::{CycleState, InputType};

use std::time::{Duration, Instant};

use itertools::Itertools;

fn part1(input: &InputType) -> i32 {
    let get_signal = |start: u32, start_val: i32, end: u32| -> Option<i32> {
        let target = (end as u32 - 20) / 40 * 40 + 20;
        if (start..end).contains(&target) {
            Some(target as i32 * start_val)
        } else {
            None
        }
    };

    let middle: i32 = input
        .windows(2)
        .filter(|pair| pair[1].start - 1 >= 20 && pair[1].start - 1 <= 220)
        .filter_map(|pair| get_signal(pair[0].start, pair[0].value, pair[1].start))
        .sum();

    let last_state = input.last().unwrap();
    let end = get_signal(last_state.start, last_state.value, last_state.start + 1).unwrap_or(0);

    middle + end
}

fn part2(input: &InputType) -> String {
    let mut states_iter = input.iter();

    let initial = CycleState::initial();
    let mut state_next = &initial;
    let mut val_current = initial.value;

    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    const SPRITE_SIZE: i32 = 3;
    let get_pixel = |i: usize, register: i32| -> char {
        let position = (i - 1) % WIDTH + 1;
        if (register..register + SPRITE_SIZE).contains(&(position as i32)) {
            '#'
        } else {
            '.'
        }
    };
    (1..=WIDTH * HEIGHT)
        .map(|i| {
            if state_next.start == i as u32 {
                val_current = state_next.value;
                state_next = states_iter.next().unwrap();
            }
            get_pixel(i, val_current)
        })
        .chunks(WIDTH)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
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
        assert_eq!(part1(&input), 13140);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        let expected_output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_string();
        assert_eq!(part2(&input), expected_output);
    }
}
