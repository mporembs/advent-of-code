use glam::IVec2;
use itertools::Itertools;
use petgraph::algo::Measure;
use petgraph::visit::{EdgeRef, IntoEdges, Visitable};
use petgraph::{stable_graph::NodeIndex, Graph};
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

//
//
//
//  DOESN'T PASS TEST BUT GOT STAR ???
//
//
//

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &'static str) -> String {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    let mut graph: Graph<IVec2, u32> = Graph::new();

    let mut grid = input
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars().enumerate().filter_map(move |(x_index, ch)| {
                let tread_char: Tread = match ch {
                    '.' => Tread::Flat,
                    'v' => Tread::Slope(SlopeDir::Down),
                    '>' => Tread::Slope(SlopeDir::Right),
                    '<' => Tread::Slope(SlopeDir::Left),
                    _ => Tread::Rock,
                };
                if let Tread::Rock = tread_char {
                    return None;
                }
                let pos = IVec2::new(x_index as i32, y_index as i32);
                Some(GardenNode {
                    position: pos,
                    tread: tread_char,
                    graph_idx: None,
                })
            })
        })
        .collect_vec();

    grid.iter_mut().for_each(|garden_node| {
        // let g_idx = graph.add_node(GardenNode.position);
        garden_node.graph_idx = Some(graph.add_node(garden_node.position));
    });

    let start = grid.iter().next().unwrap().graph_idx;
    let goal = grid.iter().last().unwrap().graph_idx;

    (0..num_rows).for_each(|row_num| {
        grid.iter()
            .filter(|g_node| g_node.position.y == row_num as i32)
            .tuple_windows::<(_, _, _)>()
            .for_each(|(node_a, node_b, node_c)| {
                let slopes = (&node_a.tread, &node_b.tread, &node_c.tread);
                // println!("=========================================");
                // dbg!(node_a.graph_idx, node_b.graph_idx, node_c.graph_idx);
                match slopes {
                    (Tread::Flat, Tread::Flat, Tread::Flat) => {
                        if node_b.position.x - node_a.position.x == 1 {
                            // println!("FFF --- Adding a -> b");

                            graph.update_edge(
                                node_a.graph_idx.unwrap(),
                                node_b.graph_idx.unwrap(),
                                1u32,
                            );
                            graph.update_edge(
                                node_b.graph_idx.unwrap(),
                                node_a.graph_idx.unwrap(),
                                1u32,
                            );
                        }
                        if node_c.position.x - node_b.position.x == 1 {
                            // println!("FFF --- Adding a -> b");

                            graph.update_edge(
                                node_b.graph_idx.unwrap(),
                                node_c.graph_idx.unwrap(),
                                1u32,
                            );
                            graph.update_edge(
                                node_c.graph_idx.unwrap(),
                                node_b.graph_idx.unwrap(),
                                1u32,
                            );
                        }
                    }
                    (Tread::Flat, Tread::Flat, Tread::Slope(_)) => {
                        if node_b.position.x - node_a.position.x == 1 {
                            // println!("FFS --- Adding a -> b");

                            graph.update_edge(
                                node_a.graph_idx.unwrap(),
                                node_b.graph_idx.unwrap(),
                                1u32,
                            );
                            graph.update_edge(
                                node_b.graph_idx.unwrap(),
                                node_a.graph_idx.unwrap(),
                                1u32,
                            );
                        }
                    }
                    (Tread::Flat, Tread::Slope(_), Tread::Flat) => {
                        if node_c.position.x - node_a.position.x == 2 {
                            // println!("FSF --- Adding a -> c");

                            graph.update_edge(
                                node_a.graph_idx.unwrap(),
                                node_c.graph_idx.unwrap(),
                                2u32,
                            );
                        }
                    }
                    (Tread::Slope(_), Tread::Flat, Tread::Flat) => {
                        if node_c.position.x - node_b.position.x == 1 {
                            // println!("SFF --- Adding b -> c");
                            graph.update_edge(
                                node_b.graph_idx.unwrap(),
                                node_c.graph_idx.unwrap(),
                                1u32,
                            );
                            graph.update_edge(
                                node_c.graph_idx.unwrap(),
                                node_b.graph_idx.unwrap(),
                                1u32,
                            );
                        }
                    }
                    _ => (),
                }
            })
    });

    (0..num_cols).for_each(|col_num| {
        grid.iter()
            .filter(|g_node| g_node.position.x == col_num as i32)
            .tuple_windows::<(_, _, _)>()
            .for_each(|(node_a, node_b, node_c)| {
                let slopes = (&node_a.tread, &node_b.tread, &node_c.tread);
                // println!("=========================================");
                // dbg!(node_a.graph_idx, node_b.graph_idx, node_c.graph_idx);

                match slopes {
                    (Tread::Flat, Tread::Flat, Tread::Flat) => {
                        if node_b.position.y - node_a.position.y == 1 {
                            // println!("FFF --- Adding a -> b, b -> c");

                            graph.update_edge(
                                node_a.graph_idx.unwrap(),
                                node_b.graph_idx.unwrap(),
                                1u32,
                            );
                            graph.update_edge(
                                node_b.graph_idx.unwrap(),
                                node_a.graph_idx.unwrap(),
                                1u32,
                            );
                        }
                        if node_c.position.y - node_b.position.y == 1 {
                            // println!("FFF --- Adding a -> b, b -> c");

                            graph.update_edge(
                                node_b.graph_idx.unwrap(),
                                node_c.graph_idx.unwrap(),
                                1u32,
                            );
                            graph.update_edge(
                                node_c.graph_idx.unwrap(),
                                node_b.graph_idx.unwrap(),
                                1u32,
                            );
                        }
                    }
                    (Tread::Flat, Tread::Flat, Tread::Slope(_)) => {
                        if node_b.position.y - node_a.position.y == 1 {
                            // println!("FFS --- Adding a -> b");

                            graph.update_edge(
                                node_a.graph_idx.unwrap(),
                                node_b.graph_idx.unwrap(),
                                1u32,
                            );
                            graph.update_edge(
                                node_b.graph_idx.unwrap(),
                                node_a.graph_idx.unwrap(),
                                1u32,
                            );
                        }
                    }
                    (Tread::Flat, Tread::Slope(_), Tread::Flat) => {
                        if node_c.position.y - node_a.position.y == 2 {
                            // println!("FSF --- Adding a -> c");

                            graph.update_edge(
                                node_a.graph_idx.unwrap(),
                                node_c.graph_idx.unwrap(),
                                2u32,
                            );
                        }
                    }
                    (Tread::Slope(_), Tread::Flat, Tread::Flat) => {
                        if node_c.position.y - node_b.position.y == 1 {
                            // println!("SFF --- Adding b -> c");

                            graph.update_edge(
                                node_b.graph_idx.unwrap(),
                                node_c.graph_idx.unwrap(),
                                1u32,
                            );
                            graph.update_edge(
                                node_c.graph_idx.unwrap(),
                                node_b.graph_idx.unwrap(),
                                1u32,
                            );
                        }
                    }
                    _ => (),
                }
            })
    });

    let count = longest_path(
        &graph,
        start.unwrap(),
        |node_idx| node_idx == goal.unwrap(),
        |edge| *edge.weight(),
        |_| 0,
    )
    .unwrap();

    dbg!(count);
    // dbg!(graph, start, goal);
    count.to_string()
}

pub fn longest_path<G, F, H, K, IsGoal>(
    graph: G,
    start: G::NodeId,
    mut is_goal: IsGoal,
    mut edge_cost: F,
    mut estimate_cost: H,
) -> Option<K>
where
    G: IntoEdges + Visitable,
    IsGoal: FnMut(G::NodeId) -> bool,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    H: FnMut(G::NodeId) -> K,
    K: Measure + Copy,
{
    let mut visit_next = BinaryHeap::new();
    let mut scores = HashMap::new(); // g-values, cost to reach the node
    let mut estimate_scores = HashMap::new(); // f-values, cost to reach + estimate cost to goal

    let zero_score = K::default();
    scores.insert(start, zero_score);
    visit_next.push(MaxScored(estimate_cost(start), start));

    while let Some(MaxScored(estimate_score, node)) = visit_next.pop() {
        if is_goal(node) {
            let cost = scores[&node];
            return Some(cost);
        }

        // This lookup can be unwrapped without fear of panic since the node was necessarily scored
        // before adding it to `visit_next`.
        let node_score = scores[&node];

        match estimate_scores.entry(node) {
            Occupied(mut entry) => {
                // If the node has already been visited with an equal or lower score than now, then
                // we do not need to re-visit it.
                if *entry.get() <= estimate_score {
                    continue;
                }
                entry.insert(estimate_score);
            }
            Vacant(entry) => {
                entry.insert(estimate_score);
            }
        }

        for edge in graph.edges(node) {
            let next = edge.target();
            let next_score = node_score + edge_cost(edge);

            match scores.entry(next) {
                Occupied(mut entry) => {
                    // No need to add neighbors that we have already reached through a shorter path
                    // than now.
                    if *entry.get() <= next_score {
                        continue;
                    }
                    entry.insert(next_score);
                }
                Vacant(entry) => {
                    entry.insert(next_score);
                }
            }

            let next_estimate_score = next_score + estimate_cost(next);
            visit_next.push(MaxScored(next_estimate_score, next));
        }
    }
    None
}

#[derive(Debug)]
enum SlopeDir {
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Tread {
    Flat,
    Slope(SlopeDir),
    Rock,
}

#[derive(Debug)]
struct GardenNode {
    position: IVec2,
    graph_idx: Option<NodeIndex>,
    tread: Tread,
}

#[derive(Debug, Clone, Copy)]
struct MaxScored<K, T>(K, T);

impl<K: PartialOrd, T> PartialEq for MaxScored<K, T> {
    #[inline]
    fn eq(&self, other: &MaxScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MaxScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MaxScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MaxScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MaxScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MaxScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(
            "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
        );
        assert_eq!(result, "94".to_string());
    }
}
