use super::load_file::load_file;
use super::matrix::{Coord, Matrix};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Visited {
    cost: Option<usize>,
    from: Option<Coord>,
    visited: bool,
}

impl Visited {
    fn new() -> Self {
        Self {
            cost: None,
            from: None,
            visited: false,
        }
    }
}

fn next_in_set(visited: &Matrix<Visited>, set: &std::collections::HashSet<Coord>) -> Option<Coord> {
    if set.is_empty() {
        return None;
    }

    let mut iter = set.iter();
    let mut result = iter.next().unwrap();
    let mut smallest = visited.get_coord(*result).cost.unwrap();

    while let Some(next) = iter.next() {
        let cost = visited.get_coord(*next).cost.unwrap();
        if cost < smallest {
            smallest = cost;
            result = next;
        }
    }

    Some(*result)
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

fn mv(coord: Coord, dir: Direction, size: usize) -> Option<Coord> {
    match dir {
        Direction::Up => {
            if coord.r > 0 {
                return Some(Coord {
                    r: coord.r - 1,
                    ..coord
                });
            }
        }
        Direction::Right => {
            if coord.c + 1 < size {
                return Some(Coord {
                    c: coord.c + 1,
                    ..coord
                });
            }
        }
        Direction::Down => {
            if coord.r + 1 < size {
                return Some(Coord {
                    r: coord.r + 1,
                    ..coord
                });
            }
        }
        Direction::Left => {
            if coord.c > 0 {
                return Some(Coord {
                    c: coord.c - 1,
                    ..coord
                });
            }
        }
    }

    None
}

fn solve(map: &Matrix<bool>) -> Option<usize> {
    let mut visited = Matrix::<Visited>::create(map.width, map.height);
    visited
        .data
        .resize(visited.width * visited.height, Visited::new());

    let mut set = std::collections::HashSet::<Coord>::new();

    visited.get_mut(0, 0).cost = Some(0);
    set.insert(Coord::create(0, 0));

    assert!(map.width == map.height);
    let end = Coord::create(map.height - 1, map.width - 1);

    while let Some(from) = next_in_set(&visited, &mut set) {
        assert!(*map.get_coord(from) == false);

        if from == end {
            return Some(visited.get_coord(from).cost.unwrap());
        }

        for dir in DIRECTIONS {
            if let Some(to) = mv(from, dir, map.width) {
                if *map.get_coord(to) == false {
                    let cost = visited.get_coord(from).cost.unwrap() + 1;
                    let neighbor_visited = visited.get_mut_coord(to);
                    if neighbor_visited
                        .cost
                        .is_none_or(|neighbor_cost| cost < neighbor_cost)
                    {
                        neighbor_visited.cost = Some(cost);
                        neighbor_visited.from = Some(from);
                        if !neighbor_visited.visited {
                            neighbor_visited.visited = true;
                            set.insert(to);
                        }
                    }
                }
            }
        }

        set.remove(&from);
    }

    None
}

fn preprocess(size: usize, bytes: usize, input: String) -> Matrix<bool> {
    let mut map = Matrix::<bool>::create(size, size);
    map.data.resize(size * size, false);
    for coord in input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut parts = line.split(",");
            let mut coord = Coord::zero();
            coord.c = parts.next().unwrap().parse::<usize>().unwrap();
            coord.r = parts.next().unwrap().parse::<usize>().unwrap();
            assert!(parts.next().is_none());
            coord
        })
        .take(bytes)
    {
        *map.get_mut_coord(coord) = true;
    }
    map
}

pub fn part1((size, bytes, input): (usize, usize, String)) -> usize {
    let map = preprocess(size, bytes, input);
    solve(&map).unwrap()
}

pub fn part2((size, _bytes, input): (usize, usize, String)) -> String {
    let mut map = Matrix::<bool>::create(size, size);
    map.data.resize(size * size, false);
    let mut bytes_falling = input.lines().filter(|x| !x.is_empty()).map(|line| {
        let mut parts = line.split(",");
        let mut coord = Coord::zero();
        coord.c = parts.next().unwrap().parse::<usize>().unwrap();
        coord.r = parts.next().unwrap().parse::<usize>().unwrap();
        assert!(parts.next().is_none());
        coord
    });

    while let Some(next) = bytes_falling.next() {
        assert!(*map.get_coord(next) == false);
        *map.get_mut_coord(next) = true;
        if solve(&map).is_none() {
            return format!("{},{}", next.c, next.r);
        }
    }
    return String::new();
}

pub fn test_input() -> (usize, usize, String) {
    (7, 12, load_file("res/day_18_test_input.txt"))
}

pub fn input() -> (usize, usize, String) {
    (71, 1024, load_file("res/day_18_input.txt"))
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 22;
    static PART1_EXPECTED_RESULT: usize = 292;
    static PART2_TEST_EXPECTED_RESULT: &str = "6,1";
    static PART2_EXPECTED_RESULT: &str = "58,44";

    #[test]
    fn part1_with_test_input() {
        assert_eq!(part1(test_input()), PART1_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input() {
        assert_eq!(part2(test_input()), PART2_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }
}
