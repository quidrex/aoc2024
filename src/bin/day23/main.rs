use aoc2024::{aoc_day, AocDay};
use itertools::Itertools;
use std::mem;

aoc_day!(Input, "7", "co,de,ka,ta");

#[derive(Clone)]
struct Input {
    edges: Vec<Vec<Vertex>>,
}

type Vertex = u16;

fn pack_vertex(value: [u8; 2]) -> Vertex {
    ((value[0] - 'a' as u8) as u16) * 26 + (value[1] - 'a' as u8) as u16
}

fn unpack_vertex(value: Vertex) -> [u8; 2] {
    [(value / 26) as u8 + 'a' as u8, (value % 26) as u8 + 'a' as u8]
}

impl AocDay for Input {
    fn from(input: &str) -> Self {
        let mut edges = vec![vec![]; 26 * 26];

        for edge_str in input.trim_end().split("\n") {
            let edge_slice = edge_str.as_bytes();
            let src = pack_vertex([edge_slice[0], edge_slice[1]]);
            let dst = pack_vertex([edge_slice[3], edge_slice[4]]);
            edges[src as usize].push(dst);
            edges[dst as usize].push(src);
        }

        Input { edges }
    }

    fn a(&self) -> String {
        self.find_3_cliques().len().to_string()
    }

    fn b(&self) -> String {
        let largest_max_clique = self.find_largest_max_clique();
        let mut output = String::with_capacity(largest_max_clique.len() * 3);
        for vertex in largest_max_clique {
            let unpacked_vertex = unpack_vertex(vertex);
            output.push(unpacked_vertex[0] as char);
            output.push(unpacked_vertex[1] as char);
            output.push(',');
        }
        output.pop();
        output.to_string()
    }
}

impl Input {
    fn find_3_cliques(&self) -> Vec<Vec<Vertex>> {
        let src_range = pack_vertex(['t' as u8, 'a' as u8])..=pack_vertex(['t' as u8, 'z' as u8]);
        let mut cliques = vec![];

        for src in src_range {
            let dsts = &self.edges[src as usize];
            if dsts.is_empty() {
                continue;
            }

            for &dst_a in dsts {
                for &dst_b in &self.edges[dst_a as usize] {
                    if self.edges[dst_b as usize].contains(&src) {
                        let mut vertices = vec![src, dst_a, dst_b];
                        vertices.sort();
                        cliques.push(vertices);
                    }
                }
            }
        }

        cliques.sort();
        cliques.dedup();
        cliques
    }

    fn find_largest_max_clique(&self) -> Vec<Vertex> {
        let mut last_cliques = self.find_3_cliques();
        let mut cliques = vec![];

        while last_cliques.len() > 1 {
            for last_clique in &last_cliques {
                for &src in &self.edges[last_clique[0] as usize] {
                    if last_clique.iter().all(|&dst| self.edges[src as usize].contains(&dst)) {
                        let mut clique = last_clique.clone();
                        clique.push(src);
                        clique.sort();
                        cliques.push(clique);
                    }
                }
            }

            cliques.sort();
            cliques.dedup();
            mem::swap(&mut last_cliques, &mut cliques);
            cliques.clear();
        }

        last_cliques.into_iter().next().unwrap()
    }
}
