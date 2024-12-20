use aoc2024::util::grid::{Coord, Dir, Grid};
use aoc2024::{aoc_day, AocDay};
use itertools::Itertools;
use std::collections::VecDeque;
use strum::{EnumString, IntoEnumIterator};

aoc_day!(Input, "5", "41");

#[derive(Clone)]
struct Input {
    min_savings: (i32, i32),
    grid: Grid<Location>,
}

#[derive(Copy, Clone, Eq, PartialEq, strum::Display, EnumString)]
enum Location {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "#")]
    Wall,
    #[strum(serialize = "S")]
    Start,
    #[strum(serialize = "E")]
    End,
}

impl AocDay for Input {
    fn from(input: &str) -> Self {
        let (min_savings_str, grid_str) = input.split_once("\n\n").unwrap();
        let (min_saving_a_str, min_saving_b_str) = min_savings_str.split_once(",").unwrap();
        let min_savings = (
            min_saving_a_str.parse::<i32>().unwrap(),
            min_saving_b_str.parse::<i32>().unwrap(),
        );
        let grid = Grid::from_str(grid_str);

        Input { min_savings, grid }
    }

    fn a(&self) -> String {
        let mut context = Context::new(&self);
        context.floodfill_dist_to_end_grid();
        context.bfs(2, self.min_savings.0).to_string()
    }

    fn b(&self) -> String {
        let mut context = Context::new(&self);
        context.floodfill_dist_to_end_grid();
        context.bfs(20, self.min_savings.1).to_string()
    }
}

#[derive(Clone)]
struct Context {
    start_idx: Coord,
    end_idx: Coord,
    wall_grid: Grid<bool>,
    dist_grid: Grid<i32>,
}

impl Context {
    fn new(input: &Input) -> Context {
        let dim = input.grid.dim();

        let mut start_idx = Coord(0, 0);
        let mut end_idx = Coord(0, 0);

        let wall_grid = Grid::from_vec(
            dim,
            input
                .grid
                .indexed_iter()
                .map(|(idx, &location)| match location {
                    Location::Empty => false,
                    Location::Wall => true,
                    Location::Start => {
                        start_idx = idx;
                        false
                    }
                    Location::End => {
                        end_idx = idx;
                        false
                    }
                })
                .collect_vec(),
        );
        let dist_grid = Grid::from_elem(dim, i32::MAX);

        Context {
            start_idx,
            end_idx,
            wall_grid,
            dist_grid,
        }
    }

    fn floodfill_dist_to_end_grid(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_back(self.end_idx);
        self.dist_grid[self.end_idx] = 0;

        while let Some(idx) = queue.pop_front() {
            let next_dist = self.dist_grid[idx] + 1;
            for dir in Dir::iter() {
                let next_idx = idx + dir;
                if !self.wall_grid[next_idx] && self.dist_grid.get(next_idx) == Some(&i32::MAX) {
                    queue.push_back(next_idx);
                    self.dist_grid[next_idx] = next_dist;
                }
            }
        }
    }

    fn bfs(&mut self, max_jump: isize, min_saving: i32) -> i32 {
        let mut worthwhile_jumps = 0;
        let mut visited_grid = Grid::from_elem(self.dist_grid.dim(), false);
        let mut queue = VecDeque::new();
        queue.push_back(self.start_idx);

        while let Some(idx) = queue.pop_front() {
            visited_grid[idx] = true;

            for dir in Dir::iter() {
                let next_idx = idx + dir;
                if !self.wall_grid[next_idx] && !visited_grid[next_idx] {
                    queue.push_back(next_idx);
                }
            }

            for offset in (0..=max_jump).flat_map(|offset0| {
                (0..=(max_jump - offset0)).map(move |offset1| (offset0, offset1))
            }) {
                if offset.0 + offset.1 < 2 {
                    continue;
                }

                for dir in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                    if offset.0 == 0 && dir.0 == -1 || offset.1 == 0 && dir.1 == -1 {
                        continue;
                    }

                    let next_idx = idx + Coord(dir.0 * offset.0, dir.1 * offset.1);
                    if self.wall_grid.get(next_idx) != Some(&false) {
                        continue;
                    }

                    let next_dist = self.dist_grid[next_idx];
                    let jump_dist = (offset.0 + offset.1) as i32;
                    let jumped_dist = self.dist_grid[idx] - next_dist;
                    if jumped_dist - jump_dist >= min_saving {
                        worthwhile_jumps += 1;
                    }
                }
            }
        }

        worthwhile_jumps
    }
}
