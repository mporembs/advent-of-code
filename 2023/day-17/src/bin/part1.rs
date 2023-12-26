use itertools::Itertools;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{EdgeRef, VisitMap, Visitable};
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

#[derive(Debug, Clone, Copy)]

struct Node {
    x: usize,
    y: usize,
    weight: u8,
}

#[derive(Debug, Clone)]
struct NodeData {
    node: NodeIndex,
    sequence: Vec<EdgeDirection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]

enum EdgeDirection {
    Right,
    Down,
    Left,
    Up,
}

impl EdgeDirection {
    fn reverse(&self) -> EdgeDirection {
        match &self {
            EdgeDirection::Right => EdgeDirection::Left,
            EdgeDirection::Down => EdgeDirection::Up,
            EdgeDirection::Left => EdgeDirection::Right,
            EdgeDirection::Up => EdgeDirection::Down,
        }
    }
}

impl<'a> fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl fmt::Display for EdgeDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let input = include_str!(r"./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    let mut graph: Graph<Node, EdgeDirection> = Graph::new();

    let nodes = input
        .lines()
        .enumerate()
        .map(|(y_index, line)| {
            line.chars()
                .enumerate()
                .map(|(x_index, ch)| {
                    graph.add_node(Node {
                        x: x_index,
                        y: y_index,
                        weight: ch.to_digit(10).unwrap() as u8,
                    })
                })
                .collect_vec()
        })
        .collect_vec();

    let start = *nodes.iter().flatten().next().unwrap();
    let goal = *nodes.iter().flatten().last().unwrap();

    let right_edges = nodes
        .iter()
        .map(|node_index_vec| {
            node_index_vec
                .windows(2)
                .map(|window| {
                    graph.add_edge(window[0], window[1], EdgeDirection::Right);
                    graph.add_edge(window[1], window[0], EdgeDirection::Left);
                })
                .collect_vec()
        })
        .collect_vec();

    let down_edges = nodes
        .iter()
        .tuple_windows::<(_, _)>()
        .flat_map(|vec_tuples| vec_tuples.0.iter().zip(vec_tuples.1).collect_vec())
        .map(|tuple| {
            graph.add_edge(*tuple.0, *tuple.1, EdgeDirection::Down);
            graph.add_edge(*tuple.1, *tuple.0, EdgeDirection::Up);
        })
        .collect_vec();

    let mut visited = graph.visit_map();
    let mut scores = HashMap::new();
    //let mut predecessor = HashMap::new();
    let mut visit_next = BinaryHeap::new();
    scores.insert(start, 0);
    visit_next.push(MinScored(
        0,
        NodeData {
            node: start,
            sequence: vec![],
        },
    ));

    while let Some(MinScored(node_score, node_data)) = visit_next.pop() {
        if visited.is_visited(&node_data.node) {
            continue;
        }

        if &goal == &node_data.node {
            break;
        }

        // Check all the edges starting at this node
        println!(
            "Node# {:?} ------------------------",
            &node_data.node.index()
        );
        for edge in graph.edges(node_data.node) {
            let next = edge.target();

            if visited.is_visited(&next) {
                println!("  Edge to {:?} already visited", next);
                continue;
            }

            let next_direction = edge.weight();
            println!("  Edge {} to {:?}", next_direction, next);
            // Clone the array of steps we've taken to get to the node we're currn
            let mut curr_sequence = node_data.sequence.clone();

            // Push in a step for coming into the starting node. NOT SURE if this is correct
            // if curr_sequence.is_empty() {
            //     curr_sequence.push(*next_direction);
            // }

            // Push the direction for this step into the copy of the previous array.
            curr_sequence.push(*next_direction);

            let len = curr_sequence.len();
            if len > 3 {
                let l3 = &curr_sequence[len - 3..];
                if l3.iter().all_equal() {
                    println!("    Illegal Move");
                    continue;
                }
            }
            let next_score = node_score + graph.node_weight(next).unwrap().weight;
            println!(
                "    Loss Value {}, Movement Sequence {:?}",
                next_score, curr_sequence
            );
            match scores.entry(next) {
                Occupied(ent) => {
                    if next_score < *ent.get() {
                        println!(
                            "      Over writing a score of {} with lower score {}",
                            *ent.get(),
                            next_score
                        );
                        *ent.into_mut() = next_score;
                        visit_next.push(MinScored(
                            next_score,
                            NodeData {
                                node: next,
                                sequence: curr_sequence,
                            },
                        ));
                        //predecessor.insert(next.clone(), node.clone());
                    }
                    println!("      A lower score already exists.");
                }
                Vacant(ent) => {
                    println!("      Inserting score");
                    ent.insert(next_score);
                    visit_next.push(MinScored(
                        next_score,
                        NodeData {
                            node: next,
                            sequence: curr_sequence,
                        },
                    ));
                    //predecessor.insert(next.clone(), node.clone());
                }
            }
        }
        println!("");

        visited.visit(node_data.node);
        // dbg!(&visit_next);
    }

    // let mut current = goal;
    // let mut path = Vec::new();
    // path.push(goal);

    // while let Some(&previous) = came_from.get(&current) {
    //     path.push(previous);
    //     current = previous;
    // }

    // path.reverse();

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeIndexLabel]));
    // dbg!(&scores);
    dbg!(&scores.len());
    dbg!(scores.get(&goal));
    // dbg!(graph);
    "0".to_string()
}

#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
// fn get_source_weight(
//     ed: EdgeReference<'_, EdgeDirection>,
//     graph: &Graph<Node, EdgeDirection>,
// ) -> u32 {
//     let source_index = ed.target();
//     graph.node_weight(source_index).unwrap().weight as u32
// }

// fn check_goal(
//     current_node: NodeIndex,
//     graph: &Graph<Node, EdgeDirection>,
//     goal: NodeIndex,
// ) -> bool {
//     current_node == goal
// }

// fn no_clue_heuristic<N>(nid: N) -> u32 {
//     0
// }

// fn make_graph(input: &str) -> Graph<Node, Direction> {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(result, "102".to_string());
    }
}
