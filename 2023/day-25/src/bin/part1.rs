use itertools::Itertools;
use nalgebra::DMatrix;
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{newline, space1},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &'static str) -> String {
    let (_, connections) = parse_connections(input).unwrap();
    let mut component_set: BTreeMap<&str, usize> = BTreeMap::new();

    connections.iter().for_each(|(source, targets)| {
        component_set.insert(&source, 0);
        targets.iter().for_each(|target| {
            component_set.insert(&target, 0);
        })
    });

    component_set
        .iter_mut()
        .enumerate()
        .for_each(|(index, (_, value))| {
            *value = index;
        });

    let matrix_size = component_set.len();
    let mut adj_matrix = DMatrix::repeat(matrix_size, matrix_size, 0f32);
    let mut degree_matrix = DMatrix::repeat(matrix_size, matrix_size, 0f32);

    connections.iter().for_each(|(source, target_vec)| {
        let x = component_set[source];
        target_vec.iter().for_each(|target| {
            let y = component_set[target];
            adj_matrix[(x, y)] = 1.0;
            adj_matrix[(y, x)] = 1.0;
        })
    });

    adj_matrix
        .column_sum()
        .iter()
        .enumerate()
        .for_each(|(row_idx, sum)| {
            degree_matrix[(row_idx, row_idx)] = *sum;
        });

    let lat_matrix = degree_matrix - adj_matrix;

    let eigen_decom = lat_matrix.symmetric_eigen();
    let target_eigen_val = eigen_decom
        .eigenvalues
        .iter()
        .sorted_by(|a: &&f32, b| a.partial_cmp(b).unwrap())
        .nth(1)
        .unwrap();

    let target_eigen_vect_idx = eigen_decom
        .eigenvalues
        .iter()
        .position(|eigen_v| eigen_v == target_eigen_val)
        .unwrap();

    let final_eigen_vector = eigen_decom
        .eigenvectors
        .columns(target_eigen_vect_idx, 1)
        .iter()
        .copied()
        .collect_vec();

    let final_vect_pos_count = final_eigen_vector
        .iter()
        .filter(|&&a: &&f32| a.is_sign_positive())
        .count();

    let final_vect_neg_count = matrix_size - final_vect_pos_count;

    (final_vect_pos_count * final_vect_neg_count).to_string()
}

fn collect_targets(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(space1, take(3usize))(input)
}

fn parse_components(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (rem, source) = take_until(":")(input)?;
    // dbg!(rem, source);
    let (rem2, targets) = preceded(tag(": "), collect_targets)(rem)?;
    Ok((rem2, (source, targets)))
}

fn parse_connections(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list0(newline, parse_components)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(
            "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr",
        );
        assert_eq!(result, "54".to_string());
    }
}
