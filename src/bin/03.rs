advent_of_code::solution!(3);

fn solve(input: &str, n: u32) -> u64 {
    let banks = input.lines();
    let mut result = 0;

    for bank in banks {
        let bank = bank.chars().collect::<Vec<_>>();
        let mut best_digits = vec![0; n as usize];
        let mut start = 0;
        for i in 0..n as u64 {
            let end = bank.len() as u64 - (n as u64 - i - 1);
            let mut max_dig = 0;
            let mut max_pos = start;
            for j in start..end {
                if bank[j as usize].to_digit(10).unwrap() as u64 > max_dig {
                    max_dig = bank[j as usize].to_digit(10).unwrap() as u64;
                    max_pos = j;
                }
            }

            best_digits[i as usize] = max_dig;
            start = max_pos + 1;
        }

        let joltage = best_digits
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + (10_u64.pow(n - 1 - i as u32) * x));
        result += joltage;
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = solve(input, 2);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
