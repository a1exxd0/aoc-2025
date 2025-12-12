advent_of_code::solution!(10);

fn parse_input_line(input: &str) -> Option<(Vec<bool>, Vec<Vec<u16>>, Vec<u16>)> {
    let input = input.trim();

    let bracket_start = input.find('[')?;
    let bracket_end = input.find(']')?;
    let pattern: Vec<bool> = input[bracket_start + 1..bracket_end]
        .chars()
        .map(|c| c == '#')
        .collect();

    let mut tuples: Vec<Vec<u16>> = Vec::new();
    let mut rest = &input[bracket_end + 1..];

    while let Some(paren_start) = rest.find('(') {
        let paren_end = rest.find(')')?;
        let tuple_content = &rest[paren_start + 1..paren_end];
        let values: Vec<u16> = tuple_content
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        tuples.push(values);
        rest = &rest[paren_end + 1..];
    }

    let brace_start = input.find('{')?;
    let brace_end = input.find('}')?;
    let set: Vec<u16> = input[brace_start + 1..brace_end]
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    Some((pattern, tuples, set))
}

fn board_to_str(board: &Vec<bool>) -> String {
    let to_chr = |b: bool| if b { '#' } else { '.' };
    let contents = board.iter().map(|&b| to_chr(b)).collect::<String>();

    format!("[{}]", contents)
}

fn parse_input(input: &str) -> Vec<(Vec<bool>, Vec<Vec<u16>>, Vec<u16>)> {
    input
        .lines()
        .map(|line| parse_input_line(line).unwrap())
        .collect::<Vec<_>>()
}

fn solve_line(entry: &(Vec<bool>, Vec<Vec<u16>>, Vec<u16>)) -> Option<u64> {
    let (pattern, toggles, _) = entry;
    let n_toggles = toggles.len();

    let target: u16 = pattern
        .iter()
        .enumerate()
        .filter(|&(_, b)| *b)
        .fold(0u16, |acc, (i, _)| acc | (1 << i));

    let masks: Vec<u16> = toggles
        .iter()
        .map(|toggle| toggle.iter().fold(0u16, |acc, &i| acc | (1 << i)))
        .collect();

    for count in 0..=n_toggles {
        for subset in 0..(1u16 << n_toggles) {
            if (subset.count_ones() as usize) != count {
                continue;
            }

            let result = (0..n_toggles)
                .filter(|&i| (subset >> i) & 1 == 1)
                .fold(0u16, |acc, i| acc ^ masks[i]);

            if result == target {
                return Some(count as u64);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let entries = parse_input(input);
    entries
        .iter()
        .map(|entry| solve_line(entry).unwrap())
        .fold(0, |acc, x| acc + x)
        .into()
}

fn solve_line_lp(entry: &(Vec<bool>, Vec<Vec<u16>>, Vec<u16>)) -> Option<u64> {
    use good_lp::*;

    let (pattern, toggles, required) = entry;
    let required = required.clone();
    if required.len() < pattern.len() {
        println!(
            "[{}]",
            required
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        panic!("bad required for line {}", board_to_str(&pattern));
    }

    let n_positions = pattern.len();
    let n_toggles = toggles.len();

    let mut problem = ProblemVariables::new();
    let presses: Vec<Variable> = (0..n_toggles)
        .map(|_| problem.add(variable().integer().min(0)))
        .collect();

    let objective: Expression = presses.iter().sum();

    let mut solver = problem.minimise(objective).using(default_solver);
    for pos in 0..n_positions {
        let affecting: Expression = toggles
            .iter()
            .enumerate()
            .filter(|(_, toggle)| toggle.contains(&(pos as u16)))
            .map(|(i, _)| presses[i])
            .sum();

        solver = solver.with(affecting.eq(required[pos] as f64));
    }

    match solver.solve() {
        Ok(solution) => {
            let total = presses
                .iter()
                .map(|&xi| solution.value(xi).round() as u64)
                .sum();
            println!("found solution for board {}: ", board_to_str(&pattern));
            Some(total)
        }
        Err(_) => {
            println!("no solution for board {}: ", board_to_str(&pattern));
            None
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let entries = parse_input(input);

    entries
        .iter()
        .map(|entry| solve_line_lp(entry).unwrap())
        .fold(0, |acc, x| acc + x)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
