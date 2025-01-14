use super::load_file::load_file;

fn preprocess(input: String) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut answer_and_nums = line.split(": ");
            let answer = answer_and_nums.next().unwrap().parse::<usize>().unwrap();
            let nums = answer_and_nums
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            assert!(answer_and_nums.next().is_none());
            (answer, nums)
        })
        .collect()
}

fn num_digits_base10(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        1 + num_digits_base10(x / 10)
    }
}

fn combine(x: usize, y: usize) -> usize {
    x * (10 as usize).pow(num_digits_base10(y) as u32) + y
}

pub fn part1(input: String) -> usize {
    fn is_valid(goal: usize, acc: usize, xs: &[usize]) -> bool {
        if xs.is_empty() {
            acc == goal
        } else if acc > goal {
            false
        } else {
            is_valid(goal, acc * xs[0], &xs[1..]) || is_valid(goal, acc + xs[0], &xs[1..])
        }
    }

    let data = preprocess(input);
    data.iter()
        .filter(|(goal, xs)| is_valid(*goal, xs[0], &xs[1..]))
        .map(|(goal, _)| goal)
        .sum()
}

pub fn part2(input: String) -> usize {
    fn is_valid(goal: usize, acc: usize, xs: &[usize]) -> bool {
        if xs.is_empty() {
            acc == goal
        } else if acc > goal {
            false
        } else {
            is_valid(goal, acc * xs[0], &xs[1..])
                || is_valid(goal, acc + xs[0], &xs[1..])
                || is_valid(goal, combine(acc, xs[0]), &xs[1..])
        }
    }

    let data = preprocess(input);
    data.iter()
        .filter(|(goal, xs)| is_valid(*goal, xs[0], &xs[1..]))
        .map(|(goal, _)| goal)
        .sum()
}

pub fn test_input() -> String {
    load_file("res/day_07_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_07_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 3749;
    static PART1_EXPECTED_RESULT: usize = 1260333054159;
    static PART2_TEST_EXPECTED_RESULT: usize = 11387;
    static PART2_EXPECTED_RESULT: usize = 162042343638683;

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
