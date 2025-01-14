use super::load_file::load_file;
use super::matrix::{Coord, Matrix};

const EMPTY_CHAR: u8 = b'.';
const WALL_CHAR: u8 = b'#';
const ROBOT_CHAR: u8 = b'@';
const BOX_CHAR: u8 = b'O';
const BOX_START_CHAR: u8 = b'[';
const BOX_END_CHAR: u8 = b']';

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opp(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

fn preprocess_moves(s: &str) -> Vec<Direction> {
    s.bytes()
        .filter_map(|x| match x {
            b'<' => Some(Direction::Left),
            b'>' => Some(Direction::Right),
            b'^' => Some(Direction::Up),
            b'v' => Some(Direction::Down),
            _ => None,
        })
        .collect::<Vec<Direction>>()
}

fn preprocess(input: String) -> (Matrix<u8>, Coord, Vec<Direction>) {
    let mut chunks = input.split("\n\n");
    let mut robot = Coord::zero();
    let map = Matrix::create_from_string_with_context_and_indices(
        chunks.next().unwrap(),
        &mut robot,
        |robot, ch, r, c| match ch {
            b'@' => {
                robot.r = r;
                robot.c = c;
                ROBOT_CHAR
            }
            b'#' => WALL_CHAR,
            b'O' => BOX_CHAR,
            _ => EMPTY_CHAR,
        },
    );
    let moves = preprocess_moves(chunks.next().unwrap());
    (map, robot, moves)
}

fn preprocess2(input: String) -> (Matrix<u8>, Coord, Vec<Direction>) {
    let mut chunks = input.split("\n\n");
    let mut robot = Coord::zero();
    let map_iter = chunks
        .next()
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .enumerate();
    let height = map_iter.clone().count();
    let width = map_iter.clone().next().unwrap().1.len() * 2;
    let mut map = Matrix::<u8>::create(width, height);
    for (r, line) in map_iter {
        for (c, ch) in line.bytes().enumerate() {
            match ch {
                b'@' => {
                    robot.r = r;
                    robot.c = c * 2;
                    map.data.push(ROBOT_CHAR);
                    map.data.push(EMPTY_CHAR);
                }
                b'#' => {
                    map.data.push(WALL_CHAR);
                    map.data.push(WALL_CHAR);
                }
                b'O' => {
                    map.data.push(BOX_START_CHAR);
                    map.data.push(BOX_END_CHAR);
                }
                _ => {
                    map.data.push(EMPTY_CHAR);
                    map.data.push(EMPTY_CHAR);
                }
            }
        }
    }
    let moves = preprocess_moves(chunks.next().unwrap());
    (map, robot, moves)
}

fn move_coord(width: usize, height: usize, coord: Coord, direction: Direction) -> Option<Coord> {
    match direction {
        Direction::Up => {
            if coord.c > 0 {
                Some(Coord {
                    c: coord.c,
                    r: coord.r - 1,
                })
            } else {
                None
            }
        }
        Direction::Right => {
            if (coord.c as usize) + 1 < width {
                Some(Coord {
                    c: coord.c + 1,
                    r: coord.r,
                })
            } else {
                None
            }
        }
        Direction::Down => {
            if (coord.r as usize) + 1 < height {
                Some(Coord {
                    c: coord.c,
                    r: coord.r + 1,
                })
            } else {
                None
            }
        }
        Direction::Left => {
            if coord.c > 0 {
                Some(Coord {
                    c: coord.c - 1,
                    r: coord.r,
                })
            } else {
                None
            }
        }
    }
}

fn next(x: Coord, d: Direction) -> Coord {
    match d {
        Direction::Up => Coord { r: x.r - 1, ..x },
        Direction::Right => Coord { c: x.c + 1, ..x },
        Direction::Down => Coord { r: x.r + 1, ..x },
        Direction::Left => Coord { c: x.c - 1, ..x },
    }
}

fn get_next_empty(pos: Coord, d: Direction, map: &Matrix<u8>) -> Option<Coord> {
    let mut test_pos = pos;
    let max_distance = match d {
        Direction::Up => pos.r,
        Direction::Right => map.width - 1 - pos.c,
        Direction::Down => map.height - 1 - pos.r,
        Direction::Left => pos.c,
    };

    for _ in 0..max_distance {
        test_pos = next(test_pos, d);
        let item = *map.get(test_pos.r as usize, test_pos.c as usize);
        if item == WALL_CHAR {
            return None;
        }
        if item == EMPTY_CHAR {
            return Some(test_pos);
        }
    }
    None
}

pub fn part1(input: String) -> usize {
    let (mut map, mut robot, moves) = preprocess(input);
    assert!((0..map.width).contains(&(robot.c as usize)));
    assert!((0..map.height).contains(&(robot.r as usize)));
    for m in moves {
        if let Some(next_empty) = get_next_empty(robot, m, &map) {
            let backward = m.opp();
            let mut i = next_empty;
            loop {
                let j = next(i, backward);
                *map.get_mut_coord(i) = *map.get_coord(j);

                if j == robot {
                    *map.get_mut_coord(robot) = EMPTY_CHAR;
                    robot = i;
                    break;
                }
                i = j;
            }
        }
    }
    let mut sum: usize = 0;
    for r in 0..map.height {
        for c in 0..map.width {
            let x = map.get(r, c);
            if *x == BOX_CHAR {
                sum += r * 100 + c;
            }
        }
    }
    sum
}

fn move_vertical(map: &mut Matrix<u8>, robot: &mut Coord, d: Direction) {
    if let Some(new_robot) = move_coord(map.width, map.height, *robot, d) {
        let mut set = Vec::<usize>::with_capacity(1);
        set.push(robot.c);
        if move_vertical_helper(map, robot.r, set, d) {
            *robot = new_robot;
        }
    }
}

fn move_vertical_helper(map: &mut Matrix<u8>, row: usize, cols: Vec<usize>, d: Direction) -> bool {
    let new_row = if d == Direction::Up {
        if row == 0 {
            return false;
        }
        row - 1
    } else {
        if row + 1 == map.height {
            return false;
        }
        row + 1
    };

    let mut new_cols = Vec::<usize>::with_capacity(map.width.min(cols.len() * 2));
    let mut hit_wall = false;
    for col in &cols {
        let item = *map.get(new_row, *col);
        if item == b'[' {
            assert!(!new_cols.contains(col));
            assert!(!new_cols.contains(&(*col + 1)));
            new_cols.push(*col);
            new_cols.push(col + 1);
        } else if item == b']' {
            if let Some(last) = new_cols.last() {
                if *last != *col {
                    assert!(!new_cols.contains(&(*col - 1)));
                    assert!(!new_cols.contains(col));
                    new_cols.push(*col - 1);
                    new_cols.push(*col);
                }
            } else {
                assert!(!new_cols.contains(col));
                assert!(!new_cols.contains(&(*col - 1)));
                new_cols.push(*col - 1);
                new_cols.push(*col);
            }
        } else if item == b'#' {
            hit_wall = true;
        }
    }

    if hit_wall {
        return false;
    }

    let do_swap =
        !hit_wall && (new_cols.is_empty() || move_vertical_helper(map, new_row, new_cols, d));

    if do_swap {
        for col in cols {
            let a = *map.get(row, col);
            let b = *map.get(new_row, col);
            *map.get_mut(row, col) = b;
            *map.get_mut(new_row, col) = a;
        }
        true
    } else {
        false
    }
}

pub fn print(map: &Matrix<u8>) {
    for r in 0..map.height {
        for c in 0..map.width {
            let ch = *map.get(r, c) as char;
            print!("{ch}");
        }
        println!("");
    }
}

pub fn part2(input: String) -> usize {
    let (mut map, mut robot, moves) = preprocess2(input);
    assert!((0..map.width).contains(&(robot.c as usize)));
    assert!((0..map.height).contains(&(robot.r as usize)));
    for m in moves {
        assert!(*map.get_coord(robot) == b'@');
        if m == Direction::Left || m == Direction::Right {
            if let Some(next_empty) = get_next_empty(robot, m, &map) {
                let backward = m.opp();
                let mut i = next_empty;
                loop {
                    let j = next(i, backward);
                    *map.get_mut_coord(i) = *map.get_coord(j);

                    if j == robot {
                        *map.get_mut_coord(robot) = EMPTY_CHAR;
                        robot = i;
                        break;
                    }
                    i = j;
                }
            }
        } else {
            // move vertically
            move_vertical(&mut map, &mut robot, m);
        }
    }
    let mut sum: usize = 0;
    for r in 0..map.height {
        for c in 0..map.width {
            let x = map.get(r, c);
            if *x == BOX_START_CHAR {
                let left_distance = c;
                let top_distance = r;
                sum += left_distance + top_distance * 100;
            }
        }
    }
    sum
}

pub fn test_input1() -> String {
    load_file("res/day_15_test_input1.txt")
}

pub fn test_input2() -> String {
    load_file("res/day_15_test_input2.txt")
}

pub fn input() -> String {
    load_file("res/day_15_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST1_EXPECTED_RESULT: usize = 2028;
    static PART1_TEST2_EXPECTED_RESULT: usize = 10092;
    static PART1_EXPECTED_RESULT: usize = 1349898;
    static PART2_TEST2_EXPECTED_RESULT: usize = 9021;
    static PART2_EXPECTED_RESULT: usize = 1376686;

    #[test]
    fn part1_with_test1_input() {
        assert_eq!(part1(test_input1()), PART1_TEST1_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_test2_input() {
        assert_eq!(part1(test_input2()), PART1_TEST2_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test3_input() {
        assert_eq!(part2(test_input2()), PART2_TEST2_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }
}
