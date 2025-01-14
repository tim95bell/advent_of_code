use super::load_file::load_file;

use super::matrix::{Coord, Matrix};

type Map = Matrix<bool>;

#[derive(Clone, Copy, PartialEq, Eq)]
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

fn preprocess(input: String) -> (Coord, Coord, Map) {
    let lines = input.trim().lines();
    let height = lines.clone().count();
    let width = lines.clone().next().unwrap().len();
    let mut map = Map::create(width, height);
    map.data.resize(width * height, false);
    let mut start = Coord::zero();
    let mut end = Coord::zero();
    for (r, line) in lines.enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            match ch {
                b'S' => {
                    start.r = r;
                    start.c = c;
                }
                b'E' => {
                    end.r = r;
                    end.c = c;
                }
                b'#' => {
                    *map.get_mut(r, c) = true;
                }
                _ => {}
            }
        }
    }
    (start, end, map)
}

fn next_in_set(cost: &Matrix<usize>, set: &mut std::collections::HashSet<Coord>) -> Option<Coord> {
    if set.is_empty() {
        return None;
    }

    let mut iter = set.iter();
    let mut result = iter.next().unwrap().clone();
    let mut smallest = cost.get_coord(result);
    assert!(*smallest < usize::MAX);

    while let Some(next) = iter.next() {
        let cost = cost.get_coord(*next);
        if cost < smallest {
            smallest = cost;
            result = next.clone();
        }
    }

    set.remove(&result);

    Some(result)
}

fn dijkstra_exhaustive(map: &Map, start: Coord) -> Matrix<usize> {
    let mut visited = Matrix::<bool>::create(map.width, map.height);
    visited.data.resize(map.width * map.height, false);
    let mut cost = Matrix::<usize>::create(map.width, map.height);
    cost.data.resize(cost.width * cost.height, usize::MAX);
    let mut set = std::collections::HashSet::<Coord>::new();
    set.insert(start);
    *cost.get_mut_coord(start) = 0;

    while let Some(from) = next_in_set(&cost, &mut set) {
        assert!(*map.get_coord(from) == false);
        assert!(*cost.get_coord(from) < usize::MAX);

        for dir in DIRECTIONS {
            if let Some(to) = mv(from, dir, map.width) {
                if *map.get_coord(to) == false {
                    let this_cost = cost.get_coord(from) + 1;
                    let neighbor_cost = cost.get_mut_coord(to);
                    if this_cost < *neighbor_cost {
                        *neighbor_cost = this_cost;
                        let neighbor_visited = visited.get_mut_coord(to);
                        if !*neighbor_visited {
                            *neighbor_visited = true;
                            set.insert(to);
                        }
                    }
                }
            }
        }
    }

    cost
}

pub fn part1(input: String) -> usize {
    let (start, end, map) = preprocess(input);
    let shortest_path_from_start = dijkstra_exhaustive(&map, start);
    let shortest_path_from_end = dijkstra_exhaustive(&map, end);

    let shortest_path = shortest_path_from_start.get_coord(end);
    let mut count: usize = 0;
    for r in 0..map.height {
        for c in 0..map.width {
            let from = Coord::create(r, c);
            for dir in DIRECTIONS {
                if !*map.get_coord(from) {
                    // not in a wall
                    if let Some(middle) = mv(from, dir, map.width) {
                        // can move 1 step in direction
                        if *map.get_coord(middle) {
                            // in a wall
                            if let Some(to) = mv(middle, dir, map.width) {
                                // can move another step in direction
                                if !*map.get_coord(to) {
                                    // not in a wall
                                    let cost_to = shortest_path_from_start.get_coord(from) + 2;
                                    let cost_from = shortest_path_from_end.get_coord(to);
                                    let cost = cost_to + cost_from;
                                    if cost <= *shortest_path - 100 {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn cheat(
    map: &Map,
    shortest_path: usize,
    cost_to_cheat_start: usize,
    cost_from: &Matrix<usize>,
    cheats: &mut std::collections::HashMap<usize, usize>,
    pos: Coord,
    limit: usize,
) {
    // not in a wall
    assert!(!*map.get_coord(pos));

    let mut set = std::collections::HashSet::<Coord>::new();
    let mut score = Matrix::<usize>::create(map.width, map.height);
    score.data.resize(map.width * map.height, usize::MAX);
    let mut visited = Matrix::<bool>::create(map.width, map.height);
    visited.data.resize(map.width * map.height, false);

    *score.get_mut_coord(pos) = 0;
    *visited.get_mut_coord(pos) = true;

    for dir in DIRECTIONS {
        if let Some(new_pos) = mv(pos, dir, map.width) {
            set.insert(new_pos);
            *score.get_mut_coord(new_pos) = 1;
            *visited.get_mut_coord(new_pos) = true;
        }
    }

    while let Some(next) = next_in_set(&score, &mut set) {
        let add_children = *score.get_coord(next) < limit;
        if add_children {
            // in wall
            // visit neighbors
            for dir in DIRECTIONS {
                if let Some(new_pos) = mv(next, dir, map.width) {
                    let cost = *score.get_coord(next) + 1;
                    let new_pos_cost = score.get_mut_coord(new_pos);
                    if cost < *new_pos_cost {
                        *new_pos_cost = cost;
                        if !*visited.get_coord(new_pos) {
                            *visited.get_mut_coord(new_pos) = true;
                            set.insert(new_pos);
                        }
                    }
                }
            }
        }

        if !*map.get_coord(next) {
            if *score.get_coord(next) <= limit {
                let cost =
                    cost_to_cheat_start + *score.get_coord(next) + *cost_from.get_coord(next);
                if cost <= shortest_path - 50 {
                    cheats
                        .entry(shortest_path - cost)
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                }
            }
        }
    }
}

pub fn part2(input: String) -> usize {
    let (start, end, map) = preprocess(input);
    let shortest_path_from_start = dijkstra_exhaustive(&map, start);
    let shortest_path_from_end = dijkstra_exhaustive(&map, end);

    let shortest_path = shortest_path_from_start.get_coord(end);
    let mut cheats = std::collections::HashMap::<usize, usize>::new();
    for r in 0..map.height {
        for c in 0..map.width {
            let from = Coord::create(r, c);
            if !*map.get_coord(from) {
                // not in a wall
                let cost_to_cheat_start = shortest_path_from_start.get_coord(from);
                cheat(
                    &map,
                    *shortest_path,
                    *cost_to_cheat_start,
                    &shortest_path_from_end,
                    &mut cheats,
                    from,
                    20,
                );
            }
        }
    }

    let result = cheats
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| v)
        .sum();
    result
}

pub fn test_input() -> String {
    load_file("res/day_20_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_20_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_EXPECTED_RESULT: usize = 1452;
    static PART2_EXPECTED_RESULT: usize = 999556;

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }
}
