use super::load_file::load_file;

fn pre_process(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut xs = vec![];
    let mut ys = vec![];
    for line in input.lines().filter(|x| !x.is_empty()) {
        let mut items = line.split_whitespace();
        xs.push(items.next().unwrap().parse::<i32>().unwrap());
        ys.push(items.next().unwrap().parse::<i32>().unwrap());
        assert!(items.count() == 0);
    }
    xs.sort();
    ys.sort();
    assert!(xs.len() == ys.len());
    (xs, ys)
}

pub fn part1(input: String) -> i32 {
    let (mut xs, mut ys) = pre_process(input);
    xs.sort();
    ys.sort();
    xs.iter().zip(ys).map(|(x, y)| i32::abs(x - y)).sum()
}

pub fn part2(input: String) -> i32 {
    let (xs, ys) = pre_process(input);

    let mut result: i32 = 0;
    for x in xs {
        result += x * ys.iter().filter(|y| **y == x).count() as i32;
    }

    result
}

pub fn part2_2(input: String) -> i32 {
    let (xs, ys) = pre_process(input);

    let mut x_index = 0;
    let mut y_index = 0;
    let mut result: i32 = 0;
    while x_index < xs.len() && y_index < ys.len() {
        let x_val = xs[x_index];
        let x_val_start_index: usize = x_index;
        x_index += 1;
        while x_index < xs.len() && xs[x_index] == x_val {
            x_index += 1;
        }
        let x_val_count = x_index - x_val_start_index;
        let mut start_y_index = y_index;
        while start_y_index < ys.len() && ys[start_y_index] < x_val {
            start_y_index += 1;
        }
        if start_y_index < ys.len() {
            y_index = start_y_index;
            while y_index < ys.len() && ys[y_index] == x_val {
                y_index += 1;
            }
            let y_val_count = y_index - start_y_index;
            result += x_val as i32 * x_val_count as i32 * y_val_count as i32;
        }
    }
    result
}

pub fn test_input() -> String {
    load_file("res/day_01_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_01_input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: i32 = 11;
    static PART1_EXPECTED_RESULT: i32 = 1530215;
    static PART2_TEST_EXPECTED_RESULT: i32 = 31;
    static PART2_EXPECTED_RESULT: i32 = 26800609;

    #[test]
    fn part1_with_test_input() {
        assert_eq!(part1(test_input()), PART1_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_real_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input() {
        assert_eq!(part2(test_input()), PART2_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_real_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }

    #[test]
    fn part2_2_with_test_input() {
        assert_eq!(part2_2(test_input()), PART2_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part2_2_with_real_input() {
        assert_eq!(part2_2(input()), PART2_EXPECTED_RESULT);
    }
}
