use aoc2024::util::grid::{Coord, Dir, Grid};
use aoc2024::{aoc_day, AocDay};
use std::collections::HashMap;
use strum::EnumString;

aoc_day!(Day16, 7036, 45);

struct Day16 {
    grid: Grid<Location>,
    start_idx: Coord,
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

impl AocDay for Day16 {
    fn from(input: &str) -> Self {
        let grid = Grid::from_str(input);
        let start_idx = grid
            .indexed_iter()
            .find(|(_, &location)| location == Location::Start)
            .unwrap()
            .0;

        Day16 { grid, start_idx }
    }

    fn a(&self) -> i64 {
        self.calculate().0 as i64
    }

    fn b(&self) -> i64 {
        self.calculate().1 as i64
    }
}

struct DfsAcc {
    score_grid: Grid<[i32; 4]>,
    path: Vec<Coord>,
    visited_grids_by_score: HashMap<i32, Grid<bool>>,
}

impl Day16 {
    fn calculate(&self) -> (i32, i32) {
        let mut acc = DfsAcc {
            score_grid: Grid::from_elem(self.grid.dim(), [i32::MAX; 4]),
            path: vec![],
            visited_grids_by_score: HashMap::new(),
        };

        let score = self.dfs(&mut acc, 0, self.start_idx, Dir::E);
        let count = acc.visited_grids_by_score[&score]
            .iter()
            .map(|b| *b as i32)
            .sum::<i32>();
        (score, count)
    }

    fn dfs(&self, acc: &mut DfsAcc, score: i32, idx: Coord, dir: Dir) -> i32 {
        match self.grid[idx] {
            Location::End => {
                let visited_grid = acc
                    .visited_grids_by_score
                    .entry(score)
                    .or_insert(Grid::from_elem(self.grid.dim(), false));

                visited_grid[idx] = true;
                for path_idx in &acc.path {
                    visited_grid[*path_idx] = true;
                }

                score
            }

            Location::Start | Location::Empty => {
                if acc.score_grid[idx][dir as usize] < score {
                    return i32::MAX;
                }
                acc.score_grid[idx][dir as usize] = score;

                acc.path.push(idx);
                let dirs = [dir, dir.turn(-1), dir.turn(1)];
                let mut scores = [i32::MAX; 3];
                scores[0] = self.dfs(acc, score + 1, idx + dirs[0], dirs[0]);
                scores[1] = self.dfs(acc, score + 1001, idx + dirs[1], dirs[1]);
                scores[2] = self.dfs(acc, score + 1001, idx + dirs[2], dirs[2]);
                acc.path.pop();

                *scores.iter().min().unwrap()
            }
            Location::Wall => i32::MAX,
        }
    }
}
