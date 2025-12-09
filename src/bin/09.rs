use std::collections::{BTreeSet, HashMap, VecDeque};

advent_of_code::solution!(9);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Piece {
    Empty,
    Corner,
    Edge,
    Filled,
}

fn parse_tiles(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            match (it.next(), it.next()) {
                (Some(x), Some(y)) => (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()),
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>()
}

fn area(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let dx = x2 as i64 - x1 as i64;
    let dy = y2 as i64 - y1 as i64;

    return ((dx.abs() + 1) * (dy.abs() + 1)) as u64;
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = parse_tiles(input);

    let mut result = 0;
    let n = tiles.len();

    for i in 0..n - 1 {
        for j in i + 1..n {
            let x = area(tiles[i].0, tiles[i].1, tiles[j].0, tiles[j].1);
            result = result.max(x);
        }
    }

    Some(result)
}

fn board_to_string(board: &Vec<Vec<Piece>>) -> String {
    board
        .iter()
        .map(|line| {
            line.iter()
                .map(|x| match x {
                    Piece::Filled => "X",
                    Piece::Edge => "E",
                    Piece::Corner => "C",
                    Piece::Empty => ".",
                })
                .collect::<String>()
                + "\n"
        })
        .collect::<String>()
}
fn in_board(board: &Vec<Vec<Piece>>, r: i64, c: i64) -> bool {
    return r >= 0 && c >= 0 && board.len() > r as usize && board[0].len() > c as usize;
}

fn flood_fill(board: &mut Vec<Vec<Piece>>) -> () {
    let n = board.len();
    let m = board[0].len();

    let mut result = None;
    for i in 0..n {
        for j in 0..m - 1 {
            if board[i][j] == Piece::Edge && board[i][j + 1] == Piece::Empty {
                for k in j + 2..m {
                    if board[i][k] != Piece::Empty {
                        result = Some((i as i32, j as i32 + 1));
                        break;
                    }
                }

                if result.is_some() {
                    break;
                }
            }
        }

        if result.is_some() {
            break;
        }
    }

    let (rows, cols) = (board.len() as i32, board[0].len() as i32);
    let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let start = result.unwrap();
    let mut q = VecDeque::with_capacity(rows as usize * cols as usize); // preallocate
    board[start.0 as usize][start.1 as usize] = Piece::Filled;

    q.push_back(start);
    let mut i: i64 = 0;
    while let Some((sx, sy)) = q.pop_front() {
        for (dx, dy) in &dirs {
            let (x, y) = (sx + dx, sy + dy);
            if x >= 0 && y >= 0 && x < rows && y < cols {
                let (ux, uy) = (x as usize, y as usize);
                if board[ux][uy] == Piece::Empty {
                    board[ux][uy] = Piece::Filled;
                    q.push_back((x, y));
                }
            }
        }

        i += 1;
        if i % 1000000000 == 0 {
            println!("iter: {}", i);
        }
    }
}

fn to_board(tiles: &mut Vec<(u64, u64)>) -> Vec<Vec<Piece>> {
    let lb_x = tiles.iter().min_by_key(|tile| tile.0).unwrap().0;
    let lb_y = tiles.iter().min_by_key(|tile| tile.1).unwrap().1;
    let ub_x = tiles.iter().max_by_key(|tile| tile.0).unwrap().0;
    let ub_y = tiles.iter().max_by_key(|tile| tile.1).unwrap().1;

    for tile in &mut *tiles {
        tile.0 -= lb_x;
        tile.1 -= lb_y;
    }

    let mut board =
        vec![vec![Piece::Empty; (ub_y + 1 - lb_y) as usize]; (ub_x + 1 - lb_x) as usize];

    let set_edges = |board: &mut Vec<Vec<Piece>>, t1: (u64, u64), t2: (u64, u64)| {
        if t1.0 == t2.0 {
            let (lb, ub) = (t1.1.min(t2.1), t1.1.max(t2.1));
            for i in lb..=ub {
                board[t1.0 as usize][i as usize] = Piece::Edge;
            }
        } else {
            let (lb, ub) = (t1.0.min(t2.0), t1.0.max(t2.0));
            for i in lb..=ub {
                board[i as usize][t1.1 as usize] = Piece::Edge;
            }
        }
    };

    for pair_tile in tiles.windows(2) {
        let t1 = pair_tile[0];
        let t2 = pair_tile[1];

        set_edges(&mut board, t1, t2);
    }

    {
        let (t1, t2) = (tiles[tiles.len() - 1], tiles[0]);
        set_edges(&mut board, t1, t2);
    }

    for corner in tiles {
        board[corner.0 as usize][corner.1 as usize] = Piece::Corner;
    }

    board
}

struct CompressedBoard {
    xs: Vec<u64>,
    ys: Vec<u64>,
    x_to_idx: HashMap<u64, usize>,
    y_to_idx: HashMap<u64, usize>,
    board: Vec<Vec<Piece>>,
}

impl CompressedBoard {
    pub fn compress(tiles: &Vec<(u64, u64)>) -> Self {
        let mut xs = BTreeSet::new();
        let mut ys = BTreeSet::new();

        for (x, y) in tiles.iter() {
            xs.insert(x);
            ys.insert(y);
        }

        let xs = xs.into_iter().cloned().collect::<Vec<_>>();
        let ys = ys.into_iter().cloned().collect::<Vec<_>>();
        let x_to_idx = xs
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i))
            .collect::<HashMap<_, _>>();
        let y_to_idx = ys
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i))
            .collect::<HashMap<_, _>>();

        let mut board = vec![vec![Piece::Empty; ys.len()]; xs.len()];
        let connect_adj = |board: &mut Vec<Vec<Piece>>, t1: &(u64, u64), t2: &(u64, u64)| {
            let t1 = (x_to_idx.get(&t1.0).unwrap(), y_to_idx.get(&t1.1).unwrap());
            let t2 = (x_to_idx.get(&t2.0).unwrap(), y_to_idx.get(&t2.1).unwrap());
            if t1.0 == t2.0 {
                let (&lb, &ub) = (t1.1.min(t2.1), t1.1.max(t2.1));
                for i in lb..=ub {
                    board[*t1.0 as usize][i as usize] = Piece::Edge;
                }
            } else {
                let (&lb, &ub) = (t1.0.min(t2.0), t1.0.max(t2.0));
                for i in lb..=ub {
                    board[i as usize][*t1.1 as usize] = Piece::Edge;
                }
            }
        };

        for tile_pair in tiles.windows(2) {
            connect_adj(&mut board, &tile_pair[0], &tile_pair[1]);
        }
        connect_adj(&mut board, &tiles.last().unwrap(), &tiles.first().unwrap());

        for (x, y) in tiles.iter() {
            let (&x, &y) = (x_to_idx.get(x).unwrap(), y_to_idx.get(y).unwrap());
            board[x as usize][y as usize] = Piece::Corner;
        }

        CompressedBoard {
            xs: xs,
            ys: ys,
            x_to_idx: x_to_idx,
            y_to_idx: y_to_idx,
            board,
        }
    }

    pub fn rectangle_filled(&self, t1: &(u64, u64), t2: &(u64, u64)) -> bool {
        let t1 = (
            self.x_to_idx.get(&t1.0).unwrap(),
            self.y_to_idx.get(&t1.1).unwrap(),
        );
        let t2 = (
            self.x_to_idx.get(&t2.0).unwrap(),
            self.y_to_idx.get(&t2.1).unwrap(),
        );

        let (lb_x, lb_y) = (t1.0.min(t2.0).clone(), t1.1.min(t2.1).clone());
        let (ub_x, ub_y) = (t1.0.max(t2.0).clone(), t1.1.max(t2.1).clone());

        for i in lb_x..=ub_x {
            for j in lb_y..=ub_y {
                // edge case adjacent edges?
                if self.board[i as usize][j as usize] == Piece::Empty {
                    return false;
                }
            }
        }

        true
    }

    pub fn area(&self, t1: &(u64, u64), t2: &(u64, u64)) -> u64 {
        let t1 = (
            self.x_to_idx.get(&t1.0).unwrap(),
            self.y_to_idx.get(&t1.1).unwrap(),
        );
        let t2 = (
            self.x_to_idx.get(&t2.0).unwrap(),
            self.y_to_idx.get(&t2.1).unwrap(),
        );

        let (lb_x, lb_y) = (t1.0.min(t2.0).clone(), t1.1.min(t2.1).clone());
        let (ub_x, ub_y) = (t1.0.max(t2.0).clone(), t1.1.max(t2.1).clone());

        let x_len = self.xs[ub_x] - self.xs[lb_x] + 1;
        let y_len = self.ys[ub_y] - self.ys[lb_y] + 1;

        return x_len * y_len;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles = parse_tiles(input);
    let mut compressed_board = CompressedBoard::compress(&tiles);
    flood_fill(&mut compressed_board.board);

    let mut result = 0;
    let n = tiles.len();

    for i in 0..n - 1 {
        for j in i + 1..n {
            if compressed_board.rectangle_filled(&tiles[i], &tiles[j]) {
                let x = compressed_board.area(&tiles[i], &tiles[j]);
                result = result.max(x);
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
