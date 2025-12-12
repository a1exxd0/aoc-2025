use std::collections::VecDeque;

advent_of_code::solution!(12);

struct Shapes {
    piece_counts: Vec<u64>,
    grids: Vec<((u64, u64), Vec<u64>)>,
}

fn parse_grid_line(input: &str) -> ((u64, u64), Vec<u64>) {
    let mut sections = input.split(':');
    let (grid_size, dims) = (sections.next().unwrap(), sections.next().unwrap());

    let mut size_sections = grid_size.split('x');
    let (w, h) = (
        size_sections.next().unwrap().parse::<u64>().unwrap(),
        size_sections.next().unwrap().parse::<u64>().unwrap(),
    );

    let dims = dims
        .trim()
        .split(' ')
        .map(|req| req.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    ((w, h), dims)
}

fn parse_piece(input: &str) -> u64 {
    input
        .chars()
        .map(|chr| if chr == '#' { 1 } else { 0 })
        .sum()
}

fn parse_input(input: &str) -> Shapes {
    let mut segments = input.split("\n\n").collect::<VecDeque<_>>();
    let grids = segments.pop_back().unwrap();
    let pieces = segments;

    let grids = grids
        .lines()
        .map(|line| parse_grid_line(line))
        .collect::<Vec<_>>();

    let pieces = pieces
        .iter()
        .map(|segment| parse_piece(segment))
        .collect::<Vec<_>>();

    Shapes {
        piece_counts: pieces,
        grids: grids,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (pieces, grids) = {
        let input = parse_input(input);
        (input.piece_counts, input.grids)
    };

    grids
        .iter()
        .map(|grid| {
            let (w, h) = grid.0;
            let expected = &grid.1;

            if expected.iter().sum::<u64>() * 8 < w * h {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
