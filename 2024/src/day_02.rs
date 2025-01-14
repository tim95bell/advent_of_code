use super::load_file::load_file;

fn valid(increasing: bool, a: i32, b: i32) -> bool {
    if increasing {
        b > a && b - a >= 1 && b - a <= 3
    } else {
        a > b && a - b >= 1 && a - b <= 3
    }
}

fn line_is_valid<T>(iter: &mut T) -> bool
where
    T: Iterator<Item = i32> + Clone,
{
    let first = iter.next().unwrap();
    let second = iter.clone().next().unwrap();
    let increasing = second > first;
    let valid = |a: &i32, b: &i32| -> bool { valid(increasing, *a, *b) };
    valid(&first, &second) && iter.is_sorted_by(valid)
}

pub fn part1(input: String) -> i32 {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut iter = line.split(' ').map(|x| x.parse::<i32>().unwrap());
            line_is_valid(&mut iter)
        })
        .filter(|x| *x)
        .count() as i32
}

pub fn part2(input: String) -> i32 {
    input
        .lines()
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            let iter = line.split(' ').map(|x| x.parse::<i32>().unwrap());
            if line_is_valid(&mut iter.clone()) {
                return true;
            }

            return (0..iter.clone().count()).into_iter().any(|index| {
                line_is_valid(
                    &mut iter
                        .clone()
                        .enumerate()
                        .filter(|(i, _)| *i != index)
                        .map(|(_, x)| x),
                )
            });
        })
        .count() as i32
}

pub fn part2_2(input: String) -> i32 {
    input
        .lines()
        .filter(|line| {
            if line.is_empty() {
                return false;
            }

            let xs = line
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            if xs.len() <= 1 {
                return true;
            }
            let increasing = xs[1] > xs[0];
            for j in 1..xs.len() {
                let i = j - 1;
                if !valid(increasing, xs[i], xs[j]) {
                    if i == 0 {
                        // remove 0 or 1
                        return check(&xs[1..]) || check2(xs[0], &xs[2..]);
                    }
                    if i == 1 {
                        // remove 1 or 2
                        return check(&xs[1..])
                            || check2(xs[0], &xs[2..])
                            || check3(increasing, xs[1], &xs[3..]);
                    }
                    return check3(increasing, xs[i - 1], &xs[j..])
                        || check3(increasing, xs[i], &xs[j + 1..]);
                }
            }
            true
        })
        .count() as i32
}

fn check(xs: &[i32]) -> bool {
    if xs.len() == 0 {
        return true;
    }

    return check2(xs[0], &xs[1..]);
}

fn check2(y: i32, xs: &[i32]) -> bool {
    if xs.len() == 0 {
        return true;
    }

    let increasing = xs[0] > y;
    return check3(increasing, y, xs);
}

fn check3(increasing: bool, y: i32, xs: &[i32]) -> bool {
    if xs.len() == 0 {
        return true;
    }

    if !valid(increasing, y, xs[0]) {
        return false;
    }

    for j in 1..xs.len() {
        let i = j - 1;
        if !valid(increasing, xs[i], xs[j]) {
            return false;
        }
    }
    return true;
}

pub fn test_input() -> String {
    load_file("res/day_02_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_02_input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: i32 = 2;
    static PART1_EXPECTED_RESULT: i32 = 257;
    static PART2_TEST_EXPECTED_RESULT: i32 = 4;
    static PART2_EXPECTED_RESULT: i32 = 328;

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

    #[test]
    fn part2_2_with_test_input() {
        assert_eq!(part2_2(test_input()), PART2_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part2_2_with_input() {
        assert_eq!(part2_2(input()), PART2_EXPECTED_RESULT);
    }
}
