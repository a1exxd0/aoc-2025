use std::collections::{BTreeSet, HashMap};

use ordered_float::OrderedFloat;

advent_of_code::solution!(8);

fn to_distances(input: &str) -> Option<(Vec<(u64, u64, u64)>, Vec<Vec<f64>>)> {
    let nodes = input
        .lines()
        .map(|line| {
            let mut elem = line.split(',');
            match (elem.next(), elem.next(), elem.next()) {
                (Some(x), Some(y), Some(z)) => (
                    x.parse::<u64>().unwrap(),
                    y.parse::<u64>().unwrap(),
                    z.parse::<u64>().unwrap(),
                ),
                _ => panic!("bad parse case"),
            }
        })
        .collect::<Vec<_>>();

    let n = nodes.len();
    let mut distances = vec![vec![f64::MAX; n]; n];
    for i in 0..n - 1 {
        for j in i + 1..n {
            distances[i][j] = ((nodes[i].0 as f64 - nodes[j].0 as f64).powi(2)
                + (nodes[i].1 as f64 - nodes[j].1 as f64).powi(2)
                + (nodes[i].2 as f64 - nodes[j].2 as f64).powi(2))
            .sqrt();
        }
    }

    Some((nodes, distances))
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn with_size(n: usize) -> UnionFind {
        let mut uf = UnionFind { parent: vec![0; n] };
        for i in 0..n {
            uf.parent[i] = i;
        }

        uf
    }

    pub fn find(&self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        } else {
            return self.find(self.parent[i]);
        }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let irep = self.find(i);
        let jrep = self.find(j);

        self.parent[irep] = jrep;
    }

    pub fn one_connected_component(&self) -> bool {
        let n = self.parent.len();
        let first_parent = self.find(0);

        for i in 1..n {
            if self.find(i) != first_parent {
                return false;
            }
        }

        true
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (nodes, distances) = to_distances(input)?;
    let n = nodes.len();
    let mut active = vec![false; n];

    let mut best_distances = BTreeSet::<(OrderedFloat<f64>, usize, usize)>::new();
    distances.iter().enumerate().for_each(|(i, elem)| {
        elem.iter().enumerate().for_each(|(j, d)| {
            best_distances.insert((OrderedFloat(*d), i, j));
        })
    });

    let mut uf = UnionFind::with_size(n);
    let iters = if n < 1000 { 10 } else { 1000 };
    for _ in 0..iters {
        let best_dist = best_distances.pop_first().unwrap();
        uf.union(best_dist.1, best_dist.2);
        active[best_dist.1] = true;
        active[best_dist.2] = true;
    }

    let mut dju = HashMap::<usize, u64>::new();
    for i in 0..n {
        if !active[i] {
            continue;
        }

        let parent = uf.find(i);
        *dju.entry(parent).or_insert(0) += 1;
    }

    let mut circuit_sizes = dju.values().cloned().collect::<Vec<_>>();
    circuit_sizes.sort();
    circuit_sizes.reverse();

    circuit_sizes
        .iter()
        .take(3)
        .fold(1, |acc, x| acc * x)
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (nodes, distances) = to_distances(input)?;
    let n = nodes.len();
    let mut active = vec![false; n];

    let mut best_distances = BTreeSet::<(OrderedFloat<f64>, usize, usize)>::new();
    distances.iter().enumerate().for_each(|(i, elem)| {
        elem.iter().enumerate().for_each(|(j, d)| {
            best_distances.insert((OrderedFloat(*d), i, j));
        })
    });

    let mut uf = UnionFind::with_size(n);
    let mut last_pair = None;
    while !uf.one_connected_component() {
        let best_dist = best_distances.pop_first().unwrap();
        uf.union(best_dist.1, best_dist.2);
        active[best_dist.1] = true;
        active[best_dist.2] = true;
        last_pair = Some(best_dist);
    }

    let (_, n1, n2) = last_pair.unwrap();
    Some(nodes[n1].0 * nodes[n2].0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
