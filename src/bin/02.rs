advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(',');

    let mut invalid = Vec::new();
    for range in ranges {
        let split_range = range.split('-').collect::<Vec<_>>();
        assert!(split_range.len() == 2);

        let l = split_range[0].trim().parse::<u64>().unwrap();
        let r = split_range[1].trim().parse::<u64>().unwrap();

        for i in l..=r {
            let as_str = i.to_string();
            if as_str.len() % 2 != 0 {
                continue;
            }

            let halfway = as_str.len() / 2;
            if as_str[..halfway] == as_str[halfway..] {
                invalid.push(i);
            }
        }
    }

    Some(invalid.iter().fold(0, |sum, x| sum + x))
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.split(',');

    let mut invalid = Vec::new();
    for range in ranges {
        let split_range = range.split('-').collect::<Vec<_>>();
        assert!(split_range.len() == 2);

        let l = split_range[0].trim().parse::<u64>().unwrap();
        let r = split_range[1].trim().parse::<u64>().unwrap();

        for i in l..=r {
            let as_str = i.to_string().chars().collect::<Vec<char>>();
            let mut flag = false;

            for len_segment in 1..as_str.len() + 1 / 2 {
                if as_str.len() % len_segment != 0 {
                    continue;
                }

                let mut inner_flag = true;
                for offset in 0..len_segment {
                    let expected = as_str[offset];
                    let mut idx = offset;
                    while idx < as_str.len() {
                        if as_str[idx] != expected {
                            inner_flag = false;
                            break;
                        }

                        idx += len_segment;
                    }

                    if !inner_flag {
                        break;
                    }
                }

                if inner_flag {
                    flag = true;
                    break;
                }
            }

            if flag {
                invalid.push(i);
            }
        }
    }

    Some(invalid.iter().fold(0, |sum, x| sum + x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
