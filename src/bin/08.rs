use std::collections::{BTreeSet, HashMap};

advent_of_code::solution!(8);

fn parse_nodes(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .map(|line| {
            let mut elem = line.split(',');
            match (elem.next(), elem.next(), elem.next()) {
                (Some(x), Some(y), Some(z)) => (
                    x.parse::<i64>().unwrap(),
                    y.parse::<i64>().unwrap(),
                    z.parse::<i64>().unwrap(),
                ),
                _ => panic!("bad parse case"),
            }
        })
        .collect()
}

#[inline]
fn dist_squared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn build_edges(nodes: &[(i64, i64, i64)]) -> Vec<(i64, usize, usize)> {
    let n = nodes.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n - 1 {
        for j in i + 1..n {
            edges.push((dist_squared(&nodes[i], &nodes[j]), i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);
    edges
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[i] != root {
            let next = self.parent[i];
            self.parent[i] = root;
            i = next;
        }
        root
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let irep = self.find(i);
        let jrep = self.find(j);

        if irep == jrep {
            return false;
        }

        if self.rank[irep] < self.rank[jrep] {
            self.parent[irep] = jrep;
        } else if self.rank[irep] > self.rank[jrep] {
            self.parent[jrep] = irep;
        } else {
            self.parent[jrep] = irep;
            self.rank[irep] += 1;
        }

        self.components -= 1;
        true
    }

    pub fn is_connected(&self) -> bool {
        self.components == 1
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let nodes = parse_nodes(input);
    let n = nodes.len();
    let edges = build_edges(&nodes);

    let iters = if n < 1000 { 10 } else { 1000 };

    let mut uf = UnionFind::new(n);
    let mut component_sizes: Vec<usize> = vec![1; n];
    let mut edges_added = 0;

    for (_, i, j) in edges.iter() {
        if edges_added >= iters {
            break;
        }

        let irep = uf.find(*i);
        let jrep = uf.find(*j);

        if irep != jrep {
            let new_size = component_sizes[irep] + component_sizes[jrep];
            uf.union(*i, *j);
            let new_rep = uf.find(*i);
            component_sizes[new_rep] = new_size;
        }

        edges_added += 1;
    }

    let mut active = vec![false; n];
    for (_, i, j) in edges.iter().take(iters) {
        active[*i] = true;
        active[*j] = true;
    }

    let mut seen_roots = std::collections::HashSet::new();
    let mut sizes = Vec::new();

    for i in 0..n {
        if active[i] {
            let root = uf.find(i);
            if seen_roots.insert(root) {
                sizes.push(component_sizes[root] as u64);
            }
        }
    }

    sizes.sort_unstable_by(|a, b| b.cmp(a));
    Some(sizes.iter().take(3).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let nodes = parse_nodes(input);
    let edges = build_edges(&nodes);

    let mut uf = UnionFind::new(nodes.len());
    let mut last_pair = (0, 0);

    for (_, i, j) in edges {
        if uf.union(i, j) {
            last_pair = (i, j);
            if uf.is_connected() {
                break;
            }
        }
    }

    let (n1, n2) = last_pair;
    Some((nodes[n1].0 * nodes[n2].0) as u64)
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
