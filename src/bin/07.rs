use std::collections::{BTreeSet, HashSet};

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, Hash)]
enum State {
    Splitter,
    Beam,
    Empty,
}

fn to_board(input: &str) -> Option<(Vec<Vec<State>>, usize)> {
    let board = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| match chr {
                    '.' => State::Empty,
                    'S' => State::Beam,
                    '^' => State::Splitter,
                    _ => panic!("no tr"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for col in 0..board[0].len() {
        if board[0][col] == State::Beam {
            return Some((board, col));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let (board, start) = to_board(input)?;

    let mut beams = BTreeSet::new();
    beams.insert(start);

    let mut result = 0;
    for row in 1..board.len() {
        let mut new_beams = BTreeSet::new();
        for beam in beams.iter() {
            if board[row][*beam] == State::Splitter {
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
                result += 1;
            } else {
                new_beams.insert(*beam);
            }
        }
        beams = new_beams;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (board, start) = to_board(input)?;

    let mut dp = vec![vec![0; board[0].len()]; board.len()];
    dp[board.len() - 1] = vec![1; board[0].len()];

    for row in (0..board.len() - 1).rev() {
        for col in 0..board[0].len() {
            if board[row][col] == State::Splitter {
                dp[row][col] = dp[row + 1][col - 1] + dp[row + 1][col + 1];
            } else {
                dp[row][col] = dp[row + 1][col];
            }
        }
    }

    Some(dp[0][start])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
