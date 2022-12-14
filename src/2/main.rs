mod input_processor;
use input_processor::{InputSubType, InputType, RoundOutcome};

use std::time::{Duration, Instant};

const OUTCOMES: InputType = [
    [
        RoundOutcome::Draw as i32, // Rock vs Rock
        RoundOutcome::Loss as i32, // Rock vs Paper
        RoundOutcome::Win as i32,  // Rock vs Scissors
    ],
    [
        RoundOutcome::Win as i32,
        RoundOutcome::Draw as i32,
        RoundOutcome::Loss as i32,
    ],
    [
        RoundOutcome::Loss as i32,
        RoundOutcome::Win as i32,
        RoundOutcome::Draw as i32,
    ],
];

// 2d array with rows representing RoundOutcome::L/D/W and cols representing Shape::R/P/S
const SHAPE_POINTS_FOR_OUTCOME: InputType = [[3, 1, 2], [1, 2, 3], [2, 3, 1]]; // or generate with v = (j + i + n - 1) % n + 1

fn get_subtotals_iter(input: &InputType) -> impl Iterator<Item = i32> + '_ {
    input.into_iter().map(|&row| row.into_iter().sum::<i32>())
}

fn part1(input: &InputType) -> i32 {
    let points_outcomes: i32 = lib::math::dot_product_2d(input.into_iter(), OUTCOMES.iter());
    let points_shape: InputSubType = core::array::from_fn(|i| i as i32 + 1);
    let points_shapes: i32 =
        lib::math::dot_product_1d_move_borrow(get_subtotals_iter(input), points_shape.iter());
    points_outcomes + points_shapes
}

fn part2(input: &InputType) -> i32 {
    const POINTS_OUTCOME: InputSubType = [
        RoundOutcome::Loss as i32,
        RoundOutcome::Draw as i32,
        RoundOutcome::Win as i32,
    ];
    let points_outcomes =
        lib::math::dot_product_1d_move_borrow(get_subtotals_iter(input), POINTS_OUTCOME.iter());
    let points_shapes = lib::math::dot_product_2d(input.iter(), SHAPE_POINTS_FOR_OUTCOME.iter());
    points_outcomes + points_shapes
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
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 12);
    }
}
