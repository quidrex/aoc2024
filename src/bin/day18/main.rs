use aoc2024::util::grid::{Coord, Dir, Grid};
use aoc2024::{aoc_day, AocDay};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use strum::IntoEnumIterator;

aoc_day!(Input, "22", "6,1");

#[derive(Clone)]
struct Input {
    dim: isize,
    initial_bytes_dropped: usize,
    bytes: Vec<Coord>,
}

impl AocDay for Input {
    fn from(input: &str) -> Self {
        let (metadata_str, bytes_str) = input.split_once("\n\n").unwrap();
        let (dim_str, initial_bytes_dropped_str) = metadata_str.split_once(",").unwrap();
        let dim = dim_str.parse::<isize>().unwrap() + 1;
        let initial_bytes_dropped = initial_bytes_dropped_str.parse::<usize>().unwrap();

        let bytes = bytes_str
            .trim_end()
            .split("\n")
            .map(|byte_str| {
                let (x_str, y_str) = byte_str.split_once(",").unwrap();
                Coord(
                    y_str.parse::<isize>().unwrap(),
                    x_str.parse::<isize>().unwrap(),
                )
            })
            .collect_vec();

        Input {
            dim,
            initial_bytes_dropped,
            bytes,
        }
    }

    fn a(&self) -> String {
        Context::new(&self, self.initial_bytes_dropped)
            .dijkstra()
            .to_string()
    }

    fn b(&self) -> String {
        let mut context = Context::new(&self, self.initial_bytes_dropped);

        loop {
            let dist = context.dijkstra();
            if dist == u32::MAX {
                let byte = self.bytes[context.bytes_dropped - 1];
                return format!("{},{}", byte.1, byte.0);
            }
            context.drop_byte();
        }
    }
}

#[derive(Clone)]
struct Context {
    start_idx: Coord,
    end_idx: Coord,
    bytes: Vec<Coord>,
    bytes_dropped: usize,
    wall_grid: Grid<bool>,
    dist_grid: Grid<u32>,
    heap_blueprint: BinaryHeap<Node>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    dist: u32,
    idx: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then(self.idx.0.cmp(&other.idx.0))
            .then(self.idx.1.cmp(&other.idx.1))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Context {
    fn new(input: &Input, initial_bytes_dropped: usize) -> Context {
        let start_idx = Coord(0, 0);
        let end_idx = Coord(input.dim - 1, input.dim - 1);

        let bytes = input.bytes.clone();
        let bytes_dropped = initial_bytes_dropped;

        let mut wall_grid = Grid::from_elem(Coord(input.dim, input.dim), false);

        for byte in &bytes[0..initial_bytes_dropped] {
            wall_grid[*byte] = true;
        }

        let dist_grid = Grid::from_elem(wall_grid.dim(), Default::default());

        let mut heap_blueprint = BinaryHeap::new();
        for row in 0..input.dim {
            for col in 0..input.dim {
                let idx = Coord(row, col);
                let dist = if idx == start_idx { 0 } else { u32::MAX };
                heap_blueprint.push(Node { dist: dist, idx })
            }
        }

        Context {
            start_idx,
            end_idx,
            bytes,
            bytes_dropped,
            wall_grid,
            dist_grid,
            heap_blueprint,
        }
    }

    fn drop_byte(&mut self) {
        self.wall_grid[self.bytes[self.bytes_dropped]] = true;
        self.bytes_dropped += 1;
    }

    fn dijkstra(&mut self) -> u32 {
        let mut heap = self.heap_blueprint.clone();

        self.dist_grid.iter_mut().for_each(|dist| *dist = u32::MAX);
        self.dist_grid[self.start_idx] = 0;

        while let Some(node) = heap.pop() {
            if self.dist_grid[node.idx] == u32::MAX {
                return u32::MAX;
            }

            if node.idx == self.end_idx {
                return self.dist_grid[node.idx];
            }

            if self.dist_grid[node.idx] != node.dist {
                continue;
            }

            let dist = self.dist_grid[node.idx] + 1;
            for dir in Dir::iter() {
                let idx = node.idx + dir;
                if self.wall_grid.get(idx) == Some(&false) {
                    if dist < self.dist_grid[idx] {
                        self.dist_grid[idx] = dist;
                        heap.push(Node { dist: dist, idx })
                    }
                }
            }
        }

        u32::MAX
    }
}
