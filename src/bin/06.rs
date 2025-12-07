advent_of_code::solution!(6);

#[derive(PartialEq, Eq)]
enum Op {
    Plus,
    Mul,
}

fn arrange_input(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
    let lines = input.lines().collect::<Vec<_>>();
    let op_line = lines.last().unwrap();
    let nums_lines = lines.iter().take(lines.len() - 1);

    let ops = op_line
        .split_whitespace()
        .map(|op| match op.trim() {
            "*" => Op::Mul,
            "+" => Op::Plus,
            _ => panic!("bad case {}", op),
        })
        .collect::<Vec<_>>();

    let mut nums = vec![vec![]; ops.len()];
    for nums_line in nums_lines {
        let num_split = nums_line
            .split_whitespace()
            .map(|num| num.trim().parse::<u64>().unwrap());

        for (i, num) in num_split.enumerate() {
            nums[i].push(num);
        }
    }

    (nums, ops)
}

fn arrange_input_2(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
    let lines = input.lines().collect::<Vec<_>>();
    let op_line = lines.last().unwrap();
    let nums_lines = lines.iter().take(lines.len() - 1);

    let mut ops = Vec::new();
    for (i, chr) in op_line.chars().enumerate() {
        if chr == ' ' {
            continue;
        }

        ops.push((
            i,
            match chr {
                '*' => Op::Mul,
                '+' => Op::Plus,
                _ => panic!("found bad chr"),
            },
        ));
    }

    let mut nums_rows = Vec::new();
    for _ in 0..ops.len() {
        nums_rows.push((vec![], 0));
    }

    for line in nums_lines {
        let line = line.chars().collect::<Vec<_>>();
        let mut in_line = Vec::new();
        let mut max_len = 0;
        let n = ops.len();

        for start in 0..n {
            let end = if start == n - 1 {
                line.len()
            } else {
                ops[start + 1].0
            };

            let sl = &line[ops[start].0..end];
            in_line.push(sl.to_vec());
            max_len = max_len.max(end - ops[start].0 - 1);
        }

        for i in 0..n {
            nums_rows[i].0.push(in_line[i].clone());
            nums_rows[i].1 = nums_rows[i].1.max(max_len)
        }
    }

    let mut nums = Vec::new();
    let n_col = nums_rows.len();
    for _ in 0..n_col {
        nums.push(vec![]);
    }

    for col in 0..n_col {
        let (rows, max_len) = (nums_rows[col].0.clone(), nums_rows[col].1);
        for col_inner in (0..max_len).rev() {
            let mut num = String::new();
            for row in rows.iter() {
                if row.len() > col_inner {
                    num += &row[col_inner].to_string();
                }
            }

            let res = num.trim();
            if res.is_empty() {
                continue;
            }
            nums[col].push(res.parse::<u64>().unwrap());
        }
    }

    (nums, ops.into_iter().map(|elem| elem.1).collect::<Vec<_>>())
}

fn apply_op_to_input(nums: &Vec<Vec<u64>>, ops: &Vec<Op>) -> u64 {
    nums.iter().zip(ops).fold(0, |acc, (nums, op)| {
        let first = nums[0];
        nums.iter().skip(1).fold(first, |acc, x| match op {
            Op::Mul => acc * x,
            Op::Plus => acc + x,
        }) + acc
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let (nums, ops) = arrange_input(input);

    Some(apply_op_to_input(&nums, &ops))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (nums, ops) = arrange_input_2(input);

    Some(apply_op_to_input(&nums, &ops))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
