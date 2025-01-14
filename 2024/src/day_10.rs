use super::load_file::load_file;
use super::matrix::Matrix;

type Set = std::collections::HashSet<(usize, usize)>;

trait TrailMarker {
    fn mark_trail(&mut self, end: (usize, usize));
    fn start_new_trail(&mut self);
    fn get_trail_head_score(&self) -> usize;
}

struct Part1TrailMarker {
    set: Set,
}

impl TrailMarker for Part1TrailMarker {
    fn mark_trail(&mut self, end: (usize, usize)) {
        self.set.insert(end);
    }

    fn start_new_trail(&mut self) {
        self.set.clear();
    }

    fn get_trail_head_score(&self) -> usize {
        self.set.len()
    }
}

struct Part2TrailMarker {
    count: usize,
}

impl Part2TrailMarker {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl TrailMarker for Part2TrailMarker {
    fn mark_trail(&mut self, _: (usize, usize)) {
        self.count += 1;
    }

    fn start_new_trail(&mut self) {
        self.count = 0;
    }

    fn get_trail_head_score(&self) -> usize {
        self.count
    }
}

fn follow<T>(trail_marker: &mut T, m: &Matrix<u8>, r: usize, c: usize, expected: u8)
where
    T: TrailMarker,
{
    if *m.get(r, c) == expected {
        if expected == 9 {
            trail_marker.mark_trail((r, c));
        } else {
            if r > 0 {
                // can go up
                follow(trail_marker, m, r - 1, c, expected + 1);
            }
            if r + 1 < m.height {
                // can go down
                follow(trail_marker, m, r + 1, c, expected + 1);
            }
            if c > 0 {
                // can go left
                follow(trail_marker, m, r, c - 1, expected + 1);
            }
            if c + 1 < m.width {
                // can go right
                follow(trail_marker, m, r, c + 1, expected + 1);
            }
        }
    }
}

fn score<T>(trail_marker: &mut T, m: &Matrix<u8>, r: usize, c: usize) -> usize
where
    T: TrailMarker,
{
    trail_marker.start_new_trail();
    assert!(*m.get(r, c) == 0);
    follow(trail_marker, m, r, c, 0);
    trail_marker.get_trail_head_score()
}

fn score_trails<T>(m: &Matrix<u8>, trail_marker: &mut T) -> usize
where
    T: TrailMarker,
{
    let mut count: usize = 0;
    for r in 0..m.height {
        for c in 0..m.width {
            if *m.get(r, c) == 0 {
                count += score(trail_marker, &m, r, c);
            }
        }
    }
    count
}

pub fn part1(input: String) -> usize {
    let m = Matrix::create_from_string(&input, |x| x - b'0');
    let mut trail_marker = Part1TrailMarker { set: Set::new() };
    score_trails(&m, &mut trail_marker)
}

pub fn part2(input: String) -> usize {
    let m = Matrix::create_from_string(&input, |x| x - b'0');
    let mut trail_marker = Part2TrailMarker::new();
    score_trails(&m, &mut trail_marker)
}

pub fn test_input() -> String {
    load_file("res/day_10_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_10_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 36;
    static PART1_EXPECTED_RESULT: usize = 782;
    static PART2_TEST_EXPECTED_RESULT: usize = 81;
    static PART2_EXPECTED_RESULT: usize = 1694;

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
