use super::load_file::load_file;
use super::matrix::{Coord, Matrix};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

pub fn print(map: &Matrix<u8>) {
    for r in 0..map.height {
        for c in 0..map.width {
            print!("{}", *map.get(r, c) as char);
        }
        println!();
    }
}

fn mv(width: usize, height: usize, pos: Coord, dir: Direction) -> Option<Coord> {
    match dir {
        Direction::North => {
            if pos.r > 0 {
                Some(Coord {
                    r: pos.r - 1,
                    ..pos
                })
            } else {
                None
            }
        }
        Direction::East => {
            if pos.c + 1 < width {
                Some(Coord {
                    c: pos.c + 1,
                    ..pos
                })
            } else {
                None
            }
        }
        Direction::South => {
            if pos.r + 1 < height {
                Some(Coord {
                    r: pos.r + 1,
                    ..pos
                })
            } else {
                None
            }
        }
        Direction::West => {
            if pos.c > 0 {
                Some(Coord {
                    c: pos.c - 1,
                    ..pos
                })
            } else {
                None
            }
        }
    }
}

type Map = Matrix<u8>;

fn solve(map: &Map, pos: Coord, dir: Direction, score: usize, context: &mut SolveContext) {
    if context.visited.get_coord(pos)[dir as usize].is_some_and(|x| score >= x) {
        return;
    }

    context.visited.get_mut_coord(pos)[dir as usize] = Some(score);

    if context.best_so_far.is_some_and(|x| score >= x) {
        return;
    }

    match *map.get_coord(pos) {
        b'E' => {
            if let Some(best_so_far) = &mut context.best_so_far {
                *best_so_far = (*best_so_far).min(score);
            } else {
                context.best_so_far = Some(score);
            }
        }
        b'#' => {}
        b'.' => {
            if let Some(pos) = mv(map.width, map.height, pos, dir) {
                solve(map, pos, dir, score + 1, context);
            }
            {
                let dir = dir.turn_clockwise();
                if let Some(pos) = mv(map.width, map.height, pos, dir) {
                    solve(map, pos, dir, score + 1001, context);
                }
            }
            {
                let dir = dir.turn_counter_clockwise();
                if let Some(pos) = mv(map.width, map.height, pos, dir) {
                    solve(map, pos, dir, score + 1001, context);
                }
            }
        }
        _ => {
            unreachable!()
        }
    }
}

fn solve2(map: &Map, pos: Coord, dir: Direction, score: usize, context: &mut SolveContext) {
    if context.visited.get_coord(pos)[dir as usize].is_some_and(|x| score >= x) {
        return;
    }

    context.visited.get_mut_coord(pos)[dir as usize] = Some(score);

    if context.best_so_far.is_some_and(|x| score > x) {
        return;
    }

    match *map.get_coord(pos) {
        b'E' => {
            if let Some(best_so_far) = &mut context.best_so_far {
                *best_so_far = (*best_so_far).min(score);
            } else {
                context.best_so_far = Some(score);
            }
        }
        b'#' => {}
        b'.' => {
            if let Some(pos) = mv(map.width, map.height, pos, dir) {
                solve(map, pos, dir, score + 1, context);
            }
            {
                let dir = dir.turn_clockwise();
                if let Some(pos) = mv(map.width, map.height, pos, dir) {
                    solve(map, pos, dir, score + 1001, context);
                }
            }
            {
                let dir = dir.turn_counter_clockwise();
                if let Some(pos) = mv(map.width, map.height, pos, dir) {
                    solve(map, pos, dir, score + 1001, context);
                }
            }
        }
        _ => {
            unreachable!()
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn walk_back(map: &mut Map, context: &SolveContext, pos: Coord, dir: Option<Direction>) {
    *map.get_mut_coord(pos) = b'O';
    let mut dirs = *context.visited.get_coord(pos);
    if let Some(dir) = dir {
        for i in 0..dirs.len() {
            if i != dir as usize {
                dirs[i] = dirs[i].map(|x| x + 1000);
            }
        }
    }
    let nums = dirs.iter().filter_map(|x| *x);
    if let Some(min) = nums.min() {
        for i in 0..dirs.len() {
            if dirs[i].is_some_and(|x| x == min) {
                let dir = DIRECTIONS[i];
                let opp_dir = dir.opposite();
                if let Some(new_pos) = mv(map.width, map.height, pos, opp_dir) {
                    walk_back(map, context, new_pos, Some(dir));
                }
            }
        }
    }
}

struct SolveContext {
    best_so_far: Option<usize>,
    visited: Matrix<[Option<usize>; 4]>,
}

pub fn part1(input: String) -> usize {
    let dir = Direction::East;
    let mut pos_and_end = (Coord::zero(), Coord::zero());
    let map = Matrix::create_from_string_with_context_and_indices(
        &input,
        &mut pos_and_end,
        |pos_and_end, ch, r, c| {
            if ch == b'S' {
                pos_and_end.0.r = r;
                pos_and_end.0.c = c;
                b'.'
            } else {
                if ch == b'E' {
                    pos_and_end.1.r = r;
                    pos_and_end.1.c = c;
                    b'E'
                } else {
                    ch
                }
            }
        },
    );
    let pos = pos_and_end.0;
    let mut context = SolveContext {
        best_so_far: None,
        visited: Matrix::<[Option<usize>; 4]>::create(map.width, map.height),
    };
    context
        .visited
        .data
        .resize(map.width * map.height, [None; 4]);

    solve(&map, pos, dir, 0, &mut context);
    context.best_so_far.unwrap()
}

pub fn part2(input: String) -> usize {
    let dir = Direction::East;
    let mut pos_and_end = (Coord::zero(), Coord::zero());
    let mut map = Matrix::create_from_string_with_context_and_indices(
        &input,
        &mut pos_and_end,
        |pos_and_end, ch, r, c| {
            if ch == b'S' {
                pos_and_end.0.r = r;
                pos_and_end.0.c = c;
                b'.'
            } else {
                if ch == b'E' {
                    pos_and_end.1.r = r;
                    pos_and_end.1.c = c;
                    b'E'
                } else {
                    ch
                }
            }
        },
    );
    let pos = pos_and_end.0;
    let end = pos_and_end.1;
    let mut context = SolveContext {
        best_so_far: None,
        visited: Matrix::<[Option<usize>; 4]>::create(map.width, map.height),
    };
    context
        .visited
        .data
        .resize(map.width * map.height, [None; 4]);

    solve2(&map, pos, dir, 0, &mut context);
    let xs = context.visited.get_mut_coord(pos);
    for x in xs {
        *x = None;
    }
    walk_back(&mut map, &context, end, None);
    map.data.iter().filter(|ch| **ch == b'O').count()
}

pub fn test1_input() -> String {
    load_file("res/day_16_test_input1.txt")
}

pub fn test2_input() -> String {
    load_file("res/day_16_test_input2.txt")
}

pub fn input() -> String {
    load_file("res/day_16_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST1_EXPECTED_RESULT: usize = 7036;
    static PART1_TEST2_EXPECTED_RESULT: usize = 11048;
    //static PART1_EXPECTED_RESULT: usize = 101492;
    //static PART2_EXPECTED_RESULT: usize = 543;

    #[test]
    fn part1_with_test1_input() {
        assert_eq!(part1(test1_input()), PART1_TEST1_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_test2_input() {
        assert_eq!(part1(test2_input()), PART1_TEST2_EXPECTED_RESULT);
    }

    // NOTE(TB): this stack overflows in a test
    // #[test]
    // fn part1_with_input() {
    //     assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    // }

    // NOTE(TB): this stack overflows in a test
    // #[test]
    // fn part2_with_input() {
    //     assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    // }
}
