use lib::input_parser;

use std::collections::VecDeque;

pub type InputType = HeightMap;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Position(usize, usize);

impl Position {
    const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        Position::DIRS.iter().filter_map(|&offset| {
            let next_r = self.0 as isize + offset.0;
            let next_c = self.1 as isize + offset.1;
            if next_r >= 0 && next_c >= 0 {
                return Some(Self(next_r as usize, next_c as usize));
            }
            None
        })
    }
}

pub struct HeightMap {
    elevations: Vec<Vec<u32>>,
    start: Position,
    lowest_elevation: Vec<Position>,
    end: Position,
}

impl HeightMap {
    pub fn get_shortest_distance_from_lowest(&self) -> Result<u32, &str> {
        self.get_shortest_distance(&self.lowest_elevation)
    }

    pub fn get_shortest_distance_from_start(&self) -> Result<u32, &str> {
        self.get_shortest_distance(&vec![self.start])
    }

    fn get_shortest_distance(&self, start: &Vec<Position>) -> Result<u32, &str> {
        let mut queue = VecDeque::from(start.clone());
        let nrows = self.elevations.len();
        let ncols = self.elevations[0].len();
        let mut visited = vec![vec![false; ncols]; nrows];
        let mut steps = 0;
        let is_valid = |next_r: usize, next_c: usize, elevation: u32| -> bool {
            next_r < nrows && next_c < ncols && self.elevations[next_r][next_c] <= elevation
        };
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();
                if current == self.end {
                    return Ok(steps);
                } else if !visited[current.0][current.1] {
                    visited[current.0][current.1] = true;
                    let elevation = self.elevations[current.0][current.1];
                    // println!("{:?},{:?}", current, elevation);
                    current
                        .neighbors()
                        .filter(|next| {
                            is_valid(next.0, next.1, elevation + 1) && !visited[next.0][next.1]
                        })
                        .for_each(|next| {
                            // println!("{:?}", next);
                            queue.push_back(next);
                        });
                }
            }
            steps += 1;
        }
        Err("No path found")
    }
}

pub fn get_input(file: &str) -> HeightMap {
    let mut lowest_elevation = Vec::new();
    let mut start = Position::default();
    let mut end = Position::default();
    let elevations = input_parser::read_file_to_string(file!(), file)
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, mut c)| {
                    match c {
                        'S' => {
                            start = Position(row, col);
                            lowest_elevation.push(start);
                            c = 'a'
                        }
                        'E' => {
                            end = Position(row, col);
                            c = 'z'
                        }
                        'a' => lowest_elevation.push(Position(row, col)),
                        c => {
                            if !('a'..='z').contains(&c) {
                                panic!("unexpected input")
                            } else {
                                ()
                            }
                        }
                    };
                    c as u32 - 'a' as u32
                })
                .collect::<Vec<u32>>()
        })
        .collect();
    HeightMap {
        elevations,
        start,
        end,
        lowest_elevation,
    }
}
