advent_of_code::solution!(4);

#[derive(PartialEq, Eq)]
enum Item {
    Space,
    Paper,
    Collected,
}

fn in_range(x: &Vec<Vec<Item>>, r: i32, c: i32) -> bool {
    return r >= 0 && c >= 0 && r < x.len() as i32 && c < x[0].len() as i32;
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|line| {
            line.iter()
                .map(|chr| match chr {
                    '.' => Item::Space,
                    '@' => Item::Paper,
                    _ => panic!("unexpected"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let r = lines.len();
    let c = lines[0].len();

    let mut collected = 0;
    for i in 0..r {
        for j in 0..c {
            if lines[i][j] != Item::Paper {
                continue;
            }

            let mut ctr = 0;
            for di in -1_i32..=1 {
                for dj in -1_i32..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }

                    let x = i as i32 + di;
                    let y = j as i32 + dj;
                    if !in_range(&lines, x, y) {
                        continue;
                    }

                    if lines[x as usize][y as usize] == Item::Paper {
                        ctr += 1;
                    }
                }
            }
            if ctr < 4 {
                collected += 1;
            }
        }
    }

    Some(collected)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|line| {
            line.iter()
                .map(|chr| match chr {
                    '.' => Item::Space,
                    '@' => Item::Paper,
                    _ => panic!("unexpected"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let r = lines.len();
    let c = lines[0].len();

    let mut collected = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..r {
            for j in 0..c {
                if lines[i][j] != Item::Paper {
                    continue;
                }

                let mut ctr = 0;
                for di in -1_i32..=1 {
                    for dj in -1_i32..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }

                        let x = i as i32 + di;
                        let y = j as i32 + dj;
                        if !in_range(&lines, x, y) {
                            continue;
                        }

                        if lines[x as usize][y as usize] == Item::Paper {
                            ctr += 1;
                        }
                    }
                }

                if ctr < 4 {
                    collected += 1;
                    lines[i][j] = Item::Space;
                    changed = true;
                }
            }
        }
    }

    Some(collected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
