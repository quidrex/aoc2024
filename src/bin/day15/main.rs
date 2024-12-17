mod util;

use crate::util::*;
use aoc2024::{aoc_day, AocDay};
use ndarray::prelude::*;

aoc_day!(Day15, "10092", "9021");

struct Day15 {
    warehouse: Warehouse,
    program: Program,
}

#[derive(Clone)]
struct Warehouse {
    grid: Array2<Location>,
    guard_pos: [usize; 2],
}

type Program = Vec<Dir>;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Location {
    Empty,
    Wall,
    Box,
    BoxL,
    BoxR,
}

impl AocDay for Day15 {
    fn from(input: &str) -> Self {
        let (grid_str, program_str) = input.split_once("\n\n").unwrap();

        let mut guard_pos = [0, 0];
        let grid = parse_matrix(grid_str, |pos, c| match c {
            '#' => Location::Wall,
            'O' => Location::Box,
            '@' => {
                guard_pos = pos;
                Location::Empty
            }
            '.' => Location::Empty,
            _ => panic!(),
        });
        let warehouse = Warehouse { grid, guard_pos };

        let program = program_str
            .trim_end()
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Dir::U),
                '>' => Some(Dir::R),
                'v' => Some(Dir::D),
                '<' => Some(Dir::L),
                _ => None,
            })
            .collect::<Vec<_>>();

        Day15 { program, warehouse }
    }

    fn a(&self) -> String {
        let mut clone = self.warehouse.clone();
        clone.run_program(&self.program);
        clone.get_box_coordinate_sum().to_string()
    }

    fn b(&self) -> String {
        let mut expanded = self.warehouse.expand();
        expanded.run_program(&self.program);
        expanded.get_box_coordinate_sum().to_string()
    }
}

impl Warehouse {
    pub fn expand(&self) -> Self {
        let grid_vec = self
            .grid
            .iter()
            .flat_map(|&location| match location {
                Location::Box => [Location::BoxL, Location::BoxR],
                l => [l, l],
            })
            .collect::<Vec<_>>();
        let grid_dim = (self.grid.dim().0, self.grid.dim().1 * 2);
        let grid = Array2::from_shape_vec(grid_dim, grid_vec).unwrap();

        let guard_pos = [self.guard_pos[0], self.guard_pos[1] * 2];

        Warehouse { grid, guard_pos }
    }

    pub fn get_box_coordinate_sum(&self) -> usize {
        self.grid
            .indexed_iter()
            .map(|(idx, &elem)| {
                if elem == Location::Box || elem == Location::BoxL {
                    idx.0 * 100 + idx.1
                } else {
                    0
                }
            })
            .sum::<usize>()
    }

    pub fn run_program(&mut self, program: &Program) {
        for &dir in program {
            let next_pos = self.guard_pos + dir;

            if self.can_move_boxes(next_pos, dir) {
                self.move_boxes(next_pos, dir);
            }

            if self.grid[next_pos] == Location::Empty {
                self.guard_pos = next_pos;
            }
        }
    }

    fn can_move_boxes(&self, pos: Position, dir: Dir) -> bool {
        let next_pos = pos + dir;

        match self.grid[pos] {
            Location::Empty => true,
            Location::Wall => false,
            Location::BoxL if dir == Dir::U || dir == Dir::D => {
                self.can_move_boxes(next_pos, dir) && self.can_move_boxes(next_pos + Dir::R, dir)
            }
            Location::BoxR if dir == Dir::U || dir == Dir::D => {
                self.can_move_boxes(next_pos + Dir::L, dir) && self.can_move_boxes(next_pos, dir)
            }
            _ => self.can_move_boxes(next_pos, dir),
        }
    }

    fn move_boxes(&mut self, pos: Position, dir: Dir) {
        match self.grid[pos] {
            Location::Empty => return,
            Location::Wall => panic!(),
            Location::BoxL if dir == Dir::U || dir == Dir::D => {
                self.do_move_boxes(pos + Dir::R, dir)
            }

            Location::BoxR if dir == Dir::U || dir == Dir::D => {
                self.do_move_boxes(pos + Dir::L, dir)
            }
            _ => {}
        };

        self.do_move_boxes(pos, dir);
    }

    fn do_move_boxes(&mut self, pos: Position, dir: Dir) {
        let next_pos = pos + dir;

        self.move_boxes(next_pos, dir);
        self.grid[next_pos] = self.grid[pos];
        self.grid[pos] = Location::Empty;
    }
}
