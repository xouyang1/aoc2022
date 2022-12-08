mod input_processor;
use input_processor::InputType;

use std::time::{Duration, Instant};

type OutputType = u32;

mod part1 {
    pub fn update_dp_visible_top_left(input: &super::InputType, visible: &mut Vec<Vec<bool>>) {
        let n = input[0].len();
        let mut peak_vertical: Vec<i32> = vec![-1; n + 2];
        input.iter().enumerate().for_each(|(irow, row)| {
            let mut left = -1;
            row.iter().enumerate().for_each(|(icol, &val)| {
                update_peak(
                    irow,
                    icol,
                    val as i32,
                    visible,
                    &mut peak_vertical,
                    &mut left,
                );
            })
        });
    }

    pub fn update_dp_visible_bottom_right(
        input: &super::InputType,
        visible: &mut Vec<Vec<bool>>,
    ) -> super::OutputType {
        let mut count = 0;
        let m = input.len();
        let n = input[0].len();
        let mut peak_vertical: Vec<i32> = vec![-1; n + 2];
        input.iter().rev().enumerate().for_each(|(irow, row)| {
            let mut right = -1;
            row.iter().rev().enumerate().for_each(|(icol, &val)| {
                let r = m - 1 - irow;
                let c = n - 1 - icol;
                update_peak(
                    r,
                    c,
                    val as i32,
                    visible,
                    &mut peak_vertical,
                    &mut right,
                );
                if visible[r][c] {
                    count += 1;
                }
            })
        });
        count
    }

    fn update_peak(
        irow: usize,
        icol: usize,
        val: i32,
        visible: &mut Vec<Vec<bool>>,
        peak_vertical: &mut Vec<i32>,
        horizontal: &mut i32,
    ) {
        let vertical = peak_vertical[icol + 1];
        visible[irow][icol] = visible[irow][icol] || std::cmp::min(vertical, *horizontal) < val;
        peak_vertical[icol + 1] = std::cmp::max(vertical, val);
        *horizontal = std::cmp::max(*horizontal, val);
    }
}

fn part1(input: &InputType) -> OutputType {
    let m = input.len();
    let n = input[0].len();
    let mut visible = vec![vec![false; n]; m];
    crate::part1::update_dp_visible_top_left(input, &mut visible);
    crate::part1::update_dp_visible_bottom_right(input, &mut visible)
}

fn update_closest_index_for_val(
    irow: usize,
    icol: usize,
    val: usize,
    vertical: &mut Vec<Vec<u32>>,
    horizontal: &mut Vec<u32>,
) {
    vertical[icol][..=(val)]
        .iter_mut()
        .for_each(|idx| *idx = irow as u32);
    horizontal[..=(val)]
        .iter_mut()
        .for_each(|idx| *idx = icol as u32);
}

fn part2(input: &InputType) -> OutputType {
    let m = input.len();
    let n = input[0].len();
    let mut vertical: Vec<Vec<u32>> = vec![vec![0; 10]; n];
    let mut product: Vec<Vec<u32>> = vec![vec![1; n]; m];
    let mut max_product: OutputType = 0;
    input.iter().enumerate().for_each(|(irow, row_data)| {
        let mut horizontal = vec![0; 10];
        row_data.iter().enumerate().for_each(|(icol, &val)| {
            let top = irow as u32 - vertical[icol][val as usize];
            let left = icol as u32 - horizontal[val as usize];
            product[irow][icol] = top * left;
            update_closest_index_for_val(irow, icol, val as usize, &mut vertical, &mut horizontal);
        })
    });
    vertical = vec![vec![m as u32 - 1; 10]; n];
    input
        .iter()
        .rev()
        .enumerate()
        .for_each(|(irow_rev, row_data)| {
            let irow = m - 1 - irow_rev;
            let mut horizontal = vec![n as u32 - 1; 10];
            row_data
                .iter()
                .rev()
                .enumerate()
                .for_each(|(icol_rev, &val)| {
                    let icol = n - 1 - icol_rev;
                    let bottom = vertical[icol][val as usize] - irow as u32;
                    let right = horizontal[val as usize] - icol as u32;
                    product[irow][icol] *= bottom * right;
                    max_product = std::cmp::max(max_product, product[irow][icol]);
                    update_closest_index_for_val(
                        irow,
                        icol,
                        val as usize,
                        &mut vertical,
                        &mut horizontal,
                    );
                })
        });
    max_product
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
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 8);
    }
}
