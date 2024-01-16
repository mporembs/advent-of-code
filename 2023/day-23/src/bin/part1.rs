use glam::IVec2;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::{stable_graph::NodeIndex, Graph};

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
    let all_paths = all_simple_paths::<Vec<_>, &Graph<IVec2, u32>>(
        &graph,
        start.unwrap(),
        goal.unwrap(),
        1usize,
        None,
    )
    .collect_vec();

    let longest = all_paths
        .iter()
        .map(|path| {
            path.iter()
                .tuple_windows::<(_, _)>()
                .fold(0u32, |acc, (node, node_next)| {
                    acc + graph
                        .edges_connecting(*node, *node_next)
                        .next()
                        .unwrap()
                        .weight()
                })
        })
        .max()
        .unwrap();

    longest.to_string()
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
