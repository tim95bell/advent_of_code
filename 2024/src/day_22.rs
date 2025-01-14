use super::load_file::load_file;

type Cache = std::collections::HashMap<usize, usize>;

fn mix(x: usize, y: usize) -> usize {
    x ^ y
}

fn prune(x: usize) -> usize {
    x % 16777216
}

fn next(x: usize, cache: &mut Cache) -> usize {
    if let Some(x) = cache.get(&x) {
        *x
    } else {
        let mut y = prune(mix(x, x * 64));
        y = prune(mix(y, y / 32));
        y = prune(mix(y, y * 2048));
        cache.insert(x, y);
        y
    }
}

pub fn part1(input: String) -> usize {
    let xs = input
        .trim()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut cache = Cache::new();
    let mut result: usize = 0;
    for x in xs {
        let mut x = x;
        for _ in 0..2000 {
            x = next(x, &mut cache);
        }
        result += x;
    }
    result
}

fn next_key((a, b, c, d): (isize, isize, isize, isize)) -> Option<(isize, isize, isize, isize)> {
    if d < 9 {
        Some((a, b, c, d + 1))
    } else if c < 9 {
        Some((a, b, c + 1, -9))
    } else if b < 9 {
        Some((a, b + 1, -9, -9))
    } else if a < 9 {
        Some((a + 1, -9, -9, -9))
    } else {
        None
    }
}

pub fn part2(input: String) -> usize {
    let xs = input
        .trim()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut cache = Cache::new();
    let mut diffs_to_prices = xs
        .iter()
        .map(|_| std::collections::HashMap::<(isize, isize, isize, isize), usize>::new())
        .collect::<Vec<std::collections::HashMap<(isize, isize, isize, isize), usize>>>();
    for x_index in 0..xs.len() {
        let x = xs[x_index];
        let mut last = x;
        let mut diffs: [isize; 4] = [0; 4];
        for i in 0..2000 {
            let index = i % 4;
            let new = next(last, &mut cache);

            diffs[index] = ((new % 10) as isize) - ((last % 10) as isize);
            if i >= 3 {
                let key = (
                    diffs[(index + 1) % 4],
                    diffs[(index + 2) % 4],
                    diffs[(index + 3) % 4],
                    diffs[index],
                );
                diffs_to_prices[x_index].entry(key).or_insert(new % 10);
            }

            last = new;
        }
    }

    let mut key = (-9, -9, -9, -9);
    let mut largest = 0;
    loop {
        let sum = diffs_to_prices
            .iter()
            .map(|x| x.get(&key).unwrap_or(&0))
            .sum();
        if sum > largest {
            largest = sum;
        }

        if let Some(next_key) = next_key(key) {
            key = next_key;
        } else {
            break;
        }
    }
    largest
}

pub fn test_input() -> String {
    load_file("res/day_22_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_22_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 37327623;
    static PART1_EXPECTED_RESULT: usize = 13429191512;
    static PART2_EXPECTED_RESULT: usize = 1582;

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
