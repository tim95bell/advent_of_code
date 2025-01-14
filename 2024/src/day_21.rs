use super::load_file::load_file;
use super::matrix::Coord;

trait Keypad {
    const A: Self;
    const AVOID: Coord;
    fn coord(&self) -> Coord;
}

#[derive(Clone, Copy)]
enum NumericalKeypad {
    Number(u8),
    A,
}

impl NumericalKeypad {
    fn number(&self) -> usize {
        match self {
            Self::Number(x) => *x as usize,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for NumericalKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::A => write!(f, "A"),
        }
    }
}

impl std::fmt::Debug for NumericalKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Keypad for NumericalKeypad {
    const A: Self = NumericalKeypad::A;
    const AVOID: Coord = Coord { r: 3, c: 0 };
    fn coord(&self) -> Coord {
        match self {
            NumericalKeypad::A => Coord::create(3, 2),
            NumericalKeypad::Number(0) => Coord::create(3, 1),
            NumericalKeypad::Number(1) => Coord::create(2, 0),
            NumericalKeypad::Number(2) => Coord::create(2, 1),
            NumericalKeypad::Number(3) => Coord::create(2, 2),
            NumericalKeypad::Number(4) => Coord::create(1, 0),
            NumericalKeypad::Number(5) => Coord::create(1, 1),
            NumericalKeypad::Number(6) => Coord::create(1, 2),
            NumericalKeypad::Number(7) => Coord::create(0, 0),
            NumericalKeypad::Number(8) => Coord::create(0, 1),
            NumericalKeypad::Number(9) => Coord::create(0, 2),
            NumericalKeypad::Number(_) => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum DirectionalKeypad {
    Up,
    Right,
    Down,
    Left,
    A,
}

impl std::fmt::Display for DirectionalKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::A => write!(f, "A"),
        }
    }
}

impl std::fmt::Debug for DirectionalKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Keypad for DirectionalKeypad {
    const A: Self = DirectionalKeypad::A;
    const AVOID: Coord = Coord { r: 0, c: 0 };

    fn coord(&self) -> Coord {
        match self {
            DirectionalKeypad::Up => Coord::create(0, 1),
            DirectionalKeypad::A => Coord::create(0, 2),
            DirectionalKeypad::Left => Coord::create(1, 0),
            DirectionalKeypad::Down => Coord::create(1, 1),
            DirectionalKeypad::Right => Coord::create(1, 2),
        }
    }
}

fn number_part(data: &Vec<NumericalKeypad>) -> usize {
    assert!(data.len() == 4);
    data[0].number() * 100 + data[1].number() * 10 + data[2].number()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct KeypadMove {
    from: Coord,
    to: Coord,
    depth: usize,
}

type Cache = std::collections::HashMap<KeypadMove, usize>;

fn min_cost<From: Keypad, To: Keypad>(
    cache: &mut Cache,
    from: From,
    to: To,
    depth: usize,
    max_depth: usize,
) -> usize {
    let from_coord = from.coord();
    let to_coord = to.coord();
    let key = KeypadMove {
        from: from_coord,
        to: to_coord,
        depth,
    };
    if let Some(result) = cache.get(&key) {
        return *result;
    }

    if depth == max_depth {
        cache.insert(key, 1);
        return 1;
    }

    let horizontal = if from_coord.c > to_coord.c {
        DirectionalKeypad::Left
    } else {
        DirectionalKeypad::Right
    };

    let vertical = if from_coord.r > to_coord.r {
        DirectionalKeypad::Up
    } else {
        DirectionalKeypad::Down
    };

    let mut cost: usize = 0;
    if from_coord.r == to_coord.r {
        // only move in one dimension (row)
        let mut prev_position = DirectionalKeypad::A;
        for _ in 0..from_coord.c.abs_diff(to_coord.c) {
            cost += min_cost(cache, prev_position, horizontal, depth + 1, max_depth);
            prev_position = horizontal;
        }
        cost += min_cost(
            cache,
            prev_position,
            DirectionalKeypad::A,
            depth + 1,
            max_depth,
        );
    } else if from_coord.c == to_coord.c {
        // only move in one dimension (col)
        let mut prev_position = DirectionalKeypad::A;
        for _ in 0..from_coord.r.abs_diff(to_coord.r) {
            cost += min_cost(cache, prev_position, vertical, depth + 1, max_depth);
            prev_position = vertical;
        }
        cost += min_cost(
            cache,
            prev_position,
            DirectionalKeypad::A,
            depth + 1,
            max_depth,
        );
        assert!(cost != 0);
    } else {
        let avoid = From::AVOID;
        let mut row_first_cost: usize = 0;
        let mut col_first_cost: usize = 0;
        assert!(cost == 0);
        let col_first = !(from_coord.r == avoid.r && to_coord.c == avoid.c);
        let row_first = !(from_coord.c == avoid.c && to_coord.r == avoid.r);
        assert!(col_first || row_first);
        if col_first {
            // can move col first without ending up in avoid
            let mut prev_position = DirectionalKeypad::A;
            for _ in 0..from_coord.c.abs_diff(to_coord.c) {
                col_first_cost += min_cost(cache, prev_position, horizontal, depth + 1, max_depth);
                prev_position = horizontal;
            }
            for _ in 0..from_coord.r.abs_diff(to_coord.r) {
                col_first_cost += min_cost(cache, prev_position, vertical, depth + 1, max_depth);
                prev_position = vertical;
            }
            col_first_cost += min_cost(
                cache,
                prev_position,
                DirectionalKeypad::A,
                depth + 1,
                max_depth,
            );
        }

        if row_first {
            // can move row first without ending up in avoid
            let mut prev_position = DirectionalKeypad::A;
            for _ in 0..from_coord.r.abs_diff(to_coord.r) {
                row_first_cost += min_cost(cache, prev_position, vertical, depth + 1, max_depth);
                prev_position = vertical;
            }
            for _ in 0..from_coord.c.abs_diff(to_coord.c) {
                row_first_cost += min_cost(cache, prev_position, horizontal, depth + 1, max_depth);
                prev_position = horizontal;
            }
            row_first_cost += min_cost(
                cache,
                prev_position,
                DirectionalKeypad::A,
                depth + 1,
                max_depth,
            );
        }

        cost = if col_first && row_first {
            col_first_cost.min(row_first_cost)
        } else if col_first {
            col_first_cost
        } else {
            row_first_cost
        };
    }

    cache.insert(key, cost);
    assert!(cost != 0);
    return cost;
}

fn translate(cache: &mut Cache, data: &Vec<NumericalKeypad>, max_depth: usize) -> usize {
    let mut result: usize = 0;
    let mut prev = NumericalKeypad::A;
    for key in data {
        result += min_cost(cache, prev, *key, 0, max_depth);
        prev = *key;
    }
    result * number_part(data)
}

fn translate_all(data: &Vec<Vec<NumericalKeypad>>, max_depth: usize) -> usize {
    let mut cache = Cache::new();
    let mut result: usize = 0;
    for datum in data {
        result += translate(&mut cache, datum, max_depth);
    }
    result
}

fn preprocess(input: String) -> Vec<Vec<NumericalKeypad>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .bytes()
                .map(|ch| {
                    if ch == b'A' {
                        NumericalKeypad::A
                    } else {
                        assert!(ch >= b'0');
                        let num = ch - b'0';
                        assert!(num <= 9);
                        NumericalKeypad::Number(num)
                    }
                })
                .collect::<Vec<NumericalKeypad>>()
        })
        .collect::<Vec<Vec<NumericalKeypad>>>()
}

pub fn part1(input: String) -> usize {
    let data = preprocess(input);
    translate_all(&data, 3)
}

pub fn part2(input: String) -> usize {
    let data = preprocess(input);
    translate_all(&data, 26)
}

pub fn test_input() -> String {
    load_file("res/day_21_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_21_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 126384;
    static PART1_EXPECTED_RESULT: usize = 270084;
    static PART2_EXPECTED_RESULT: usize = 329431019997766;

    #[test]
    fn part1_with_test_input() {
        assert_eq!(part1(test_input()), PART1_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }
}
