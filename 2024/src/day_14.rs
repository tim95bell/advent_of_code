use super::load_file::load_file;
use glam::IVec2;

struct Robot {
    pos: IVec2,
    vel: IVec2,
}

fn wrap(x: &mut i32, max: i32) {
    if *x >= max {
        *x -= max;
    } else if *x < 0 {
        *x += max;
    }
}

fn simulate(width: i32, height: i32, robots: &mut Vec<Robot>) {
    for robot in robots {
        robot.pos += robot.vel;
        wrap(&mut robot.pos.x, width);
        wrap(&mut robot.pos.y, height);
    }
}

fn preprocess(input: String) -> Vec<Robot> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut parts = line.split(" ");
            let mut pos_iter = parts
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|num_str| num_str.parse::<i32>().unwrap());
            let mut vel_iter = parts
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|num_str| num_str.parse::<i32>().unwrap());
            assert!(parts.next().is_none());
            Robot {
                pos: IVec2 {
                    x: pos_iter.next().unwrap(),
                    y: pos_iter.next().unwrap(),
                },
                vel: IVec2 {
                    x: vel_iter.next().unwrap(),
                    y: vel_iter.next().unwrap(),
                },
            }
        })
        .collect::<Vec<Robot>>()
}

pub fn part1((width, height, input): (i32, i32, String)) -> usize {
    let mut robots = preprocess(input);
    for _ in 0..100 {
        simulate(width, height, &mut robots);
    }

    let mut top_left: usize = 0;
    let mut top_right: usize = 0;
    let mut bottom_left: usize = 0;
    let mut bottom_right: usize = 0;

    let top_bound_a: i32 = 0;
    let top_bound_b: i32 = height / 2;
    let top_bounds = top_bound_a..top_bound_b;
    let bottom_bound_a: i32 = if height % 2 == 0 {
        top_bound_b
    } else {
        top_bound_b + 1
    };
    let bottom_bound_b: i32 = height;
    let bottom_bounds = bottom_bound_a..bottom_bound_b;

    let left_bound_a: i32 = 0;
    let left_bound_b: i32 = width / 2;
    let left_bounds = left_bound_a..left_bound_b;
    let right_bound_a: i32 = if width % 2 == 0 {
        left_bound_b
    } else {
        left_bound_b + 1
    };
    let right_bound_b: i32 = width;
    let right_bounds = right_bound_a..right_bound_b;

    for robot in robots {
        if left_bounds.contains(&robot.pos.x) {
            if top_bounds.contains(&robot.pos.y) {
                top_left += 1;
            } else if bottom_bounds.contains(&robot.pos.y) {
                bottom_left += 1;
            }
        } else if right_bounds.contains(&robot.pos.x) {
            if top_bounds.contains(&robot.pos.y) {
                top_right += 1;
            } else if bottom_bounds.contains(&robot.pos.y) {
                bottom_right += 1;
            }
        }
    }
    top_left * top_right * bottom_left * bottom_right
}

fn coord_to_index(width: i32, coord: IVec2) -> usize {
    (coord.y * width + coord.x) as usize
}

fn robots_to_grid(width: i32, robots: &Vec<Robot>, grid: &mut Vec<bool>) {
    grid.fill(false);
    for robot in robots {
        grid[coord_to_index(width, robot.pos)] = true;
    }
}

fn count_block(
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    grid: &Vec<bool>,
    visited: &mut Vec<bool>,
) -> usize {
    let index = coord_to_index(width, IVec2 { x, y });

    if visited[index] {
        return 0;
    }

    visited[index] = true;

    if !grid[index] {
        return 0;
    }

    let mut result: usize = 1;
    if x > 0 {
        result += count_block(width, height, x - 1, y, grid, visited);
    }
    if x + 1 < width {
        result += count_block(width, height, x + 1, y, grid, visited);
    }
    if y > 0 {
        result += count_block(width, height, x, y - 1, grid, visited);
    }
    if y + 1 < width {
        result += count_block(width, height, x, y + 1, grid, visited);
    }

    result
}

fn calc_score(width: i32, height: i32, grid: &Vec<bool>, visited: &mut Vec<bool>) -> usize {
    visited.fill(false);
    let mut score: usize = 0;
    visited.resize((width * height) as usize, false);
    for y in 0..height {
        for x in 0..width {
            let count = count_block(width, height, x, y, grid, visited);
            if count > 10 {
                score += count;
            }
        }
    }
    score
}

pub fn part2((width, height, input): (i32, i32, String)) -> usize {
    let mut robots = preprocess(input);
    let mut grid = Vec::<bool>::new();
    grid.resize((width * height) as usize, false);
    robots_to_grid(width, &robots, &mut grid);
    let mut visited = Vec::<bool>::new();
    let mut scores = Vec::<(usize, usize)>::new();
    let score = calc_score(width, height, &grid, &mut visited);
    scores.push((0, score));
    for i in 0..10000 {
        simulate(width, height, &mut robots);
        robots_to_grid(width, &robots, &mut grid);
        let score = calc_score(width, height, &grid, &mut visited);
        scores.push((i + 1, score));
    }
    scores.sort_by(|a, b| {
        if a.1 > b.1 {
            std::cmp::Ordering::Less
        } else if a.1 < b.1 {
            std::cmp::Ordering::Greater
        } else {
            a.0.cmp(&b.0)
        }
    });
    scores[0].0
}

pub fn test_input() -> (i32, i32, String) {
    (11, 7, load_file("res/day_14_test_input.txt"))
}

pub fn input() -> (i32, i32, String) {
    (101, 103, load_file("res/day_14_input.txt"))
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 12;
    static PART1_EXPECTED_RESULT: usize = 226179492;
    static PART2_EXPECTED_RESULT: usize = 7502;

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
