use super::load_file::load_file;
use super::matrix::Matrix;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Clone)]
enum Cell {
    Empty,
    Visited(Direction),
    Blocked,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Cell::Visited(dir) => match dir {
                Direction::Up => "^",
                Direction::Right => ">",
                Direction::Down => "V",
                Direction::Left => "<",
            },
            Cell::Empty => ".",
            Cell::Blocked => "#",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn preprocess(input: String) -> ((usize, usize), Matrix<Cell>) {
    let mut pos: (usize, usize) = (0, 0);
    let m = Matrix::create_from_string_with_context_and_indices(
        &input,
        &mut pos,
        |pos, x, r, c| match x {
            b'#' => Cell::Blocked,
            b'^' => {
                *pos = (r, c);
                Cell::Empty
            }
            _ => Cell::Empty,
        },
    );
    (pos, m)
}

fn mov(matrix: &Matrix<Cell>, pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if pos.0 > 0 {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        Direction::Right => {
            if pos.1 + 1 < matrix.width {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
        Direction::Down => {
            if pos.0 + 1 < matrix.height {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            }
        }
        Direction::Left => {
            if pos.1 > 0 {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
    }
}

pub fn part1(input: String) -> usize {
    let (mut pos, mut data) = preprocess(input);
    let mut direction = Direction::Up;
    let mut count: usize = 0;
    loop {
        let cell = data.get_mut(pos.0, pos.1);

        if *cell == Cell::Empty {
            count += 1;
            *cell = Cell::Visited(direction);
        }

        if let Some(new_pos) = mov(&data, pos, direction) {
            if *data.get(new_pos.0, new_pos.1) == Cell::Blocked {
                direction = turn_right(direction);
                continue;
            } else {
                pos = new_pos;
            }
        } else {
            break;
        }
    }
    return count;
}

fn does_loop(mut data: Matrix<Cell>, mut pos: (usize, usize), mut direction: Direction) -> bool {
    loop {
        let cell = data.get_mut(pos.0, pos.1);

        if *cell == Cell::Empty {
            *cell = Cell::Visited(direction);
        } else if let Cell::Visited(visited_direction) = *cell {
            if visited_direction == direction {
                return true;
            }
        }

        if let Some(new_pos) = mov(&data, pos, direction) {
            if *data.get(new_pos.0, new_pos.1) == Cell::Blocked {
                direction = turn_right(direction);
                continue;
            } else {
                pos = new_pos;
            }
        } else {
            return false;
        }
    }
}

pub fn part2(input: String) -> usize {
    let (mut pos, mut data) = preprocess(input);
    let mut direction = Direction::Up;
    let mut count: usize = 0;
    let initial_data = data.clone();
    let initial_pos = pos.clone();
    let initial_direction = direction.clone();
    loop {
        let cell = data.get_mut(pos.0, pos.1);

        if *cell == Cell::Empty {
            *cell = Cell::Visited(direction);
        }

        if let Some(new_pos) = mov(&data, pos, direction) {
            if *data.get(new_pos.0, new_pos.1) == Cell::Blocked {
                direction = turn_right(direction);
                continue;
            } else {
                if *data.get(new_pos.0, new_pos.1) == Cell::Empty {
                    let mut data_copy = initial_data.clone();
                    *data_copy.get_mut(new_pos.0, new_pos.1) = Cell::Blocked;
                    if does_loop(data_copy, initial_pos.clone(), initial_direction.clone()) {
                        count += 1;
                    }
                }

                pos = new_pos;
            }
        } else {
            break;
        }
    }
    return count;
}

pub fn test_input() -> String {
    load_file("res/day_06_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_06_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 41;
    static PART1_EXPECTED_RESULT: usize = 5208;
    static PART2_TEST_EXPECTED_RESULT: usize = 6;
    static PART2_EXPECTED_RESULT: usize = 1972;

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
