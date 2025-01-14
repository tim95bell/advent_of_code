use super::load_file::load_file;
use std::collections::{HashMap, HashSet};

type Map = HashMap<usize, HashSet<usize>>;

pub fn part1(input: String) -> usize {
    let mut parts = input.split("\n\n");
    let part1 = parts.next().unwrap();
    let part2 = parts.next().unwrap();
    assert!(parts.next().is_none());

    let mut map = Map::new();

    for line in part1.lines().filter(|x| !x.is_empty()) {
        let mut numbers = line.split("|");
        let x = numbers.next().unwrap().parse::<usize>().unwrap();
        let y = numbers.next().unwrap().parse::<usize>().unwrap();
        assert!(parts.next().is_none());
        map.entry(x)
            .and_modify(|set| {
                set.insert(y);
            })
            .or_insert(HashSet::from([y]));
    }

    part2
        .lines()
        .map(|line| {
            if line.is_empty() {
                return 0;
            }
            let line = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if line.is_sorted_by(|x, y| is_before(&map, *x, *y)) {
                line[line.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn is_before(map: &Map, x: usize, y: usize) -> bool {
    if let Some(set) = map.get(&x) {
        set.contains(&y)
    } else {
        false
    }
}

fn cmp(map: &Map, x: usize, y: usize) -> std::cmp::Ordering {
    if is_before(map, x, y) {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

pub fn part2(input: String) -> usize {
    let mut parts = input.split("\n\n");
    let part1 = parts.next().unwrap();
    let part2 = parts.next().unwrap();
    assert!(parts.next().is_none());

    let mut map = Map::new();

    for line in part1.lines().filter(|x| !x.is_empty()) {
        let mut numbers = line.split("|");
        let x = numbers.next().unwrap().parse::<usize>().unwrap();
        let y = numbers.next().unwrap().parse::<usize>().unwrap();
        assert!(parts.next().is_none());
        map.entry(x)
            .and_modify(|set| {
                set.insert(y);
            })
            .or_insert(HashSet::from([y]));
    }

    part2
        .lines()
        .map(|line| {
            if line.is_empty() {
                return 0;
            }
            let mut line = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if !line.is_sorted_by(|x, y| is_before(&map, *x, *y)) {
                line.sort_by(|x, y| cmp(&map, *x, *y));
                return line[line.len() / 2];
            } else {
                return 0;
            }
        })
        .sum()
}

pub fn test_input() -> String {
    load_file("res/day_05_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_05_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 143;
    static PART1_EXPECTED_RESULT: usize = 5588;
    static PART2_TEST_EXPECTED_RESULT: usize = 123;
    static PART2_EXPECTED_RESULT: usize = 5331;

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
