use glam::IVec2;
use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::Undirected;
use petgraph::{stable_graph::NodeIndex, Graph};
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parts(input);
    dbg!(output);
}

fn parts(input: &'static str) -> String {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    let mut graph: Graph<IVec2, u32, Undirected> = Graph::new_undirected();

    let mut grid = input
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars().enumerate().filter_map(move |(x_index, ch)| {
                if ch == '#' {
                    return None;
                }
                let pos = IVec2::new(x_index as i32, y_index as i32);
                Some(GardenNode {
                    position: pos,
                    graph_idx: None,
                })
            })
        })
        .collect_vec();

    grid.iter_mut().for_each(|garden_node| {
        garden_node.graph_idx = Some(graph.add_node(garden_node.position));
    });

    let start = grid.iter().next().unwrap().graph_idx;
    let goal = grid.iter().last().unwrap().graph_idx;

    (0..num_rows).for_each(|row_num| {
        grid.iter()
            .filter(|g_node| g_node.position.y == row_num as i32)
            .tuple_windows::<(_, _)>()
            .for_each(|(node_a, node_b)| {
                if node_b.position.x - node_a.position.x == 1 {
                    graph.update_edge(node_a.graph_idx.unwrap(), node_b.graph_idx.unwrap(), 1u32);
                    graph.update_edge(node_b.graph_idx.unwrap(), node_a.graph_idx.unwrap(), 1u32);
                }
            })
    });

    (0..num_cols).for_each(|col_num| {
        grid.iter()
            .filter(|g_node| g_node.position.x == col_num as i32)
            .tuple_windows::<(_, _)>()
            .for_each(|(node_a, node_b)| {
                if node_b.position.y - node_a.position.y == 1 {
                    graph.update_edge(node_a.graph_idx.unwrap(), node_b.graph_idx.unwrap(), 1u32);
                    graph.update_edge(node_b.graph_idx.unwrap(), node_a.graph_idx.unwrap(), 1u32);
                }
            })
    });

    let all_paths = all_simple_paths::<Vec<_>, &Graph<IVec2, u32, Undirected>>(
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
    // dbg!(longest);
    longest.to_string()
}

#[derive(Debug)]
struct GardenNode {
    position: IVec2,
    graph_idx: Option<NodeIndex>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = parts(
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
        assert_eq!(result, "154".to_string());
    }
}
