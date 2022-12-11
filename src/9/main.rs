#![feature(as_array_of_cells)]
#![feature(cell_update)]

mod input_processor;
use input_processor::{InputType, Motion, Position};
use itertools::Itertools;

use std::collections::HashSet;
use std::time::{Duration, Instant};

type OutputType = u32;

struct Rope<const N: usize> {
    knots: [Position; N],
    visited_tails: HashSet<Position>,
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        Self {
            knots: [(0, 0); N],
            visited_tails: HashSet::from([(0, 0)]),
        }
    }

    pub fn update_knots(&mut self, head_motion: &Motion) {
        let Motion { dir, count } = head_motion;
        for _ in 0..*count {
            if self.update_knots_and_get_has_tail_updated(dir) {
                self.visited_tails.insert(*self.knots.last().unwrap());
            }
        }
    }

    fn update_knots_and_get_has_tail_updated(&mut self, head_dir: &Position) -> bool {
        let dir = std::cell::Cell::from_mut(&mut self.knots)
            .as_array_of_cells()
            .windows(2)
            .fold_while(
                Some(*head_dir),
                |dir, pair: &[std::cell::Cell<Position>]| {
                    // pair: a pair of knots within the window that represent the current knot and the next knot
                    let dir = dir.unwrap();
                    pair[0].update(|pos| (pos.0 + dir.0, pos.1 + dir.1));
                    match Rope::<N>::get_next_adjustment(&pair[0].get(), &pair[1].get()) {
                        Some(d) => itertools::FoldWhile::Continue(Some(d)),
                        None => itertools::FoldWhile::Done(None),
                    }
                },
            )
            .into_inner();
        match dir {
            Some(d) => {
                self.knots.last_mut().unwrap().0 += d.0;
                self.knots.last_mut().unwrap().1 += d.1;
                true
            }
            None => false,
        }
    }

    fn get_next_adjustment(current: &Position, next: &Position) -> Option<Position> {
        let horizontal_dist = current.0 - next.0;
        let vertical_dist = current.1 - next.1;
        if horizontal_dist.abs() <= 1 && vertical_dist.abs() <= 1 {
            return None; // next knot does not need to be updated
        }
        Some((horizontal_dist.signum(), vertical_dist.signum()))
    }
}

fn get_unique_tail_position_count<const N: usize>(input: &InputType) -> OutputType {
    let mut rope: Rope<N> = Rope::new();
    input.iter().for_each(|motion| rope.update_knots(motion));
    rope.visited_tails.len() as OutputType
}

fn part1(input: &InputType) -> OutputType {
    get_unique_tail_position_count::<2>(input)
}

fn part2(input: &InputType) -> OutputType {
    get_unique_tail_position_count::<10>(input)
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
        assert_eq!(part1(&input), 88);
    }

    #[test]
    fn part2_test() {
        let input = input_processor::get_input("input-test.txt");
        assert_eq!(part2(&input), 36);
    }
}
