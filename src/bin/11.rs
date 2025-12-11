use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn parse_input(input: &str) -> (HashMap<String, usize>, Vec<Vec<usize>>) {
    let mut str_map = HashMap::new();
    let mut idx_map = Vec::new();

    let lines = input
        .trim()
        .lines()
        .map(|line| {
            let mut it = line.split(':');
            (it.next().unwrap(), it.next().unwrap())
        })
        .map(|(start, mappings)| {
            let parsed_mappings = mappings.trim().split(' ').collect::<Vec<_>>();
            (start.trim(), parsed_mappings)
        });

    for (start, mappings) in lines {
        let key = if let Some(key) = str_map.get(start) {
            *key
        } else {
            let key = idx_map.len();
            str_map.insert(start.to_string(), key);
            idx_map.resize(idx_map.len() + 1, vec![]);
            key
        };

        for mapping in mappings {
            let val = if let Some(val) = str_map.get(mapping) {
                *val
            } else {
                let val = idx_map.len();
                str_map.insert(mapping.to_string(), val);
                idx_map.resize(val + 1, vec![]);
                val
            };

            idx_map[key].push(val);
        }
    }

    (str_map, idx_map)
}

/// assume no cycles
fn dfs_dp(curr: usize, dest: usize, idx_map: &Vec<Vec<usize>>, dp: &mut Vec<u64>) -> u64 {
    if dp[curr] != u64::MAX {
        return dp[curr];
    }

    let outputs = &idx_map[curr];
    let result = outputs
        .iter()
        .map(|&output| dfs_dp(output, dest, idx_map, dp))
        .sum();

    dp[curr] = result;
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (str_map, idx_map) = parse_input(input);
    let out = *str_map.get("out").unwrap();
    let you = *str_map.get("you").unwrap();

    let mut dp = vec![u64::MAX; idx_map.len()];
    dp[out] = 1;

    Some(dfs_dp(you, out, &idx_map, &mut dp))
}

#[derive(Clone, Eq, PartialEq)]
struct Ctr {
    all_paths: u64,
    good_paths: u64,
    seen_fft: bool,
    seen_dac: bool,
}

impl Default for Ctr {
    fn default() -> Self {
        Ctr {
            all_paths: u64::MAX,
            good_paths: u64::MAX,
            seen_fft: false,
            seen_dac: false,
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (str_map, idx_map) = parse_input(input);
    let svr = *str_map.get("svr").unwrap();
    let out = *str_map.get("out").unwrap();
    let fft = *str_map.get("fft").unwrap();
    let dac = *str_map.get("dac").unwrap();

    let mut dp_fft_dac = vec![u64::MAX; idx_map.len()];
    dp_fft_dac[dac] = 1;
    let fft_dac = dfs_dp(fft, dac, &idx_map, &mut dp_fft_dac);

    let mut dp_svr_fft = vec![u64::MAX; idx_map.len()];
    dp_svr_fft[fft] = 1;
    let svr_fft = dfs_dp(svr, fft, &idx_map, &mut dp_svr_fft);

    let mut dp_dac_out = vec![u64::MAX; idx_map.len()];
    dp_dac_out[out] = 1;
    let dac_out = dfs_dp(dac, out, &idx_map, &mut dp_dac_out);

    Some(fft_dac * svr_fft * dac_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let result = part_one(input);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
