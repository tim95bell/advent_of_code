use super::load_file::load_file;
use std::ops::Deref;

fn preprocess(input: String) -> Vec<usize> {
    input
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

#[derive(Debug)]
pub struct Stones {
    stones: Vec<usize>,
}

impl Stones {
    pub fn create_from_string(string: String) -> Self {
        Self::new(preprocess(string))
    }

    pub fn new(stones: Vec<usize>) -> Self {
        Self { stones }
    }

    pub fn next(&mut self) {
        next_gen(&mut self.stones);
    }

    pub fn gen_count(&self, gens: usize) -> usize {
        gen_count_for_stones(&self.stones, gens)
    }
}

impl Deref for Stones {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.stones
    }
}

impl std::ops::DerefMut for Stones {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stones
    }
}

fn num_digits_base10(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        1 + num_digits_base10(x / 10)
    }
}

fn next_gen(stones: &mut Vec<usize>) {
    let count = stones.len();
    for i in 0..count {
        let stone = &mut stones[i];
        if *stone == 0 {
            *stone = 1;
        } else {
            let digits: usize = num_digits_base10(*stone);
            if digits % 2 == 0 {
                let x = 10usize.pow((digits / 2) as u32);
                let left = *stone / x;
                let right = *stone - (left * x);
                *stone = left;
                stones.push(right);
            } else {
                *stone *= 2024;
            }
        }
    }
}

fn gen_count_for_stones(stones: &Vec<usize>, gens: usize) -> usize {
    let mut cache = std::collections::HashMap::<(usize, usize), usize>::new();
    let mut result: usize = 0;
    for stone in stones {
        result += gen_count(&mut cache, *stone, gens);
    }
    result
}

fn gen_count(
    cache: &mut std::collections::HashMap<(usize, usize), usize>,
    stone: usize,
    gens: usize,
) -> usize {
    if gens == 0 {
        1
    } else if let Some(result) = cache.get(&(stone, gens)) {
        *result
    } else {
        if stone == 0 {
            let count = gen_count(cache, 1, gens - 1);
            cache.entry((stone, gens)).or_insert(count);
            count
        } else {
            let digits: usize = num_digits_base10(stone);
            if digits % 2 == 0 {
                let x = 10usize.pow((digits / 2) as u32);
                let left = stone / x;
                let right = stone - (left * x);
                let count = gen_count(cache, left, gens - 1) + gen_count(cache, right, gens - 1);
                cache.entry((stone, gens)).or_insert(count);
                count
            } else {
                let count = gen_count(cache, stone * 2024, gens - 1);
                cache.entry((stone, gens)).or_insert(count);
                count
            }
        }
    }
}

pub fn test_input1() -> String {
    load_file("res/day_11_test_input1.txt")
}

pub fn test_input2() -> String {
    load_file("res/day_11_test_input2.txt")
}

pub fn input() -> String {
    load_file("res/day_11_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    fn part1_test1_expected_result() -> Vec<Vec<usize>> {
        vec![vec![0, 1, 10, 99, 999], vec![1, 2024, 1, 0, 9, 9, 2021976]]
    }

    fn part1_test2_expected_result() -> Vec<Vec<usize>> {
        vec![
            vec![125, 17],
            vec![253000, 1, 7],
            vec![253, 0, 2024, 14168],
            vec![512072, 1, 20, 24, 28676032],
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2,
            ],
        ]
    }

    static PART1_EXPECTED_RESULT: usize = 202019;
    static PART2_EXPECTED_RESULT: usize = 239321955280205;

    fn run_test(mut expected_results: Vec<Vec<usize>>, mut results: Stones) {
        results.sort();
        expected_results[0].sort();
        assert_eq!(expected_results[0], *results);
        for expected in expected_results.iter_mut().skip(1) {
            results.next();
            results.sort();
            expected.sort();
            assert_eq!(*expected, *results);
        }
    }

    #[test]
    fn part1_with_test_input1() {
        let expected_results = part1_test1_expected_result();
        let results = Stones::create_from_string(test_input1());
        run_test(expected_results, results);
    }

    #[test]
    fn part1_with_test_input2() {
        let expected_results = part1_test2_expected_result();
        let results = Stones::create_from_string(test_input2());
        run_test(expected_results, results);
    }

    #[test]
    fn part1_with_input() {
        let mut results = Stones::create_from_string(input());
        for _ in 0..25 {
            results.next();
        }
        assert_eq!(results.len(), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input1() {
        let gens = part1_test1_expected_result().len() - 1;
        let expected_result = part1_test1_expected_result().last().unwrap().len();
        let results = Stones::create_from_string(test_input1());

        assert_eq!(results.gen_count(gens), expected_result);
    }

    #[test]
    fn part2_with_test_input2() {
        let gens = part1_test2_expected_result().len() - 1;
        let expected_result = part1_test2_expected_result().last().unwrap().len();
        let results = Stones::create_from_string(test_input2());

        assert_eq!(results.gen_count(gens), expected_result);
    }

    #[test]
    fn part2_with_input_part1_answer() {
        let expected_result = PART1_EXPECTED_RESULT;
        let results = Stones::create_from_string(input());

        assert_eq!(results.gen_count(25), expected_result);
    }

    #[test]
    fn part2_with_input() {
        let results = Stones::create_from_string(input());
        assert_eq!(results.gen_count(75), PART2_EXPECTED_RESULT);
    }
}
