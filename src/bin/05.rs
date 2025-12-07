advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = {
        let mut sections = input.split("\n\n");
        match (sections.next(), sections.next()) {
            (Some(ranges_section), Some(ingredients_section)) => {
                let ranges_lines = ranges_section
                    .lines()
                    .map(|line| {
                        let mut ranges = line.trim().split('-');
                        match (ranges.next(), ranges.next()) {
                            (Some(l), Some(r)) => {
                                (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap())
                            }
                            _ => panic!("couldnt get args"),
                        }
                    })
                    .collect::<Vec<_>>();

                let ingredients_lines = ingredients_section
                    .lines()
                    .map(|line| line.trim().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                (ranges_lines, ingredients_lines)
            }
            _ => panic!("bad format"),
        }
    };

    let result = ingredients.iter().fold(0, |acc, ingredient| {
        let in_range = ranges
            .iter()
            .any(|range| *ingredient >= range.0 && *ingredient <= range.1);

        if in_range {
            acc + 1
        } else {
            acc
        }
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges = {
        let mut sections = input.split("\n\n");
        match sections.next() {
            Some(ranges_section) => ranges_section
                .lines()
                .map(|line| {
                    let mut ranges = line.trim().split('-');
                    match (ranges.next(), ranges.next()) {
                        (Some(l), Some(r)) => {
                            (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap() + 1)
                        }
                        _ => panic!("couldnt get args"),
                    }
                })
                .collect::<Vec<_>>(),
            _ => panic!("bad format"),
        }
    };

    ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::new();
    let mut current = ranges[0];
    for next in ranges.into_iter().skip(1) {
        if next.0 <= current.1 {
            current.1 = current.1.max(next.1);
        } else {
            merged.push(current);
            current = next;
        }
    }
    merged.push(current);

    let result = merged
        .iter()
        .fold(0, |acc, range| acc + (range.1 - range.0));

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
