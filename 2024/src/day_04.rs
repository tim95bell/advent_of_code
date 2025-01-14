use super::load_file::load_file;
use super::matrix::Matrix;

fn preprocess(input: String) -> Matrix<u8> {
    let mut context: () = ();
    Matrix::<u8>::create_from_string_with_context(&input, &mut context, |_, x| x)
}

pub fn part1(input: String) -> usize {
    let data = preprocess(input);
    // right, up-right, down-right, down
    let mut count: usize = 0;
    static XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];
    static SMAX: [u8; 4] = [b'S', b'A', b'M', b'X'];
    for r in 0..data.height {
        for c in 0..data.width {
            let x = data.get(r, c);
            if *x == b'X' || *x == b'S' {
                let word = if *x == b'X' { &XMAS } else { &SMAX };
                if c + 3 < data.width {
                    // right
                    if *data.get(r, c + 1) == word[1]
                        && *data.get(r, c + 2) == word[2]
                        && *data.get(r, c + 3) == word[3]
                    {
                        count += 1;
                    }
                    if r + 3 < data.height {
                        // down
                        if *data.get(r + 1, c) == word[1]
                            && *data.get(r + 2, c) == word[2]
                            && *data.get(r + 3, c) == word[3]
                        {
                            count += 1;
                        }
                        // down-right
                        if *data.get(r + 1, c + 1) == word[1]
                            && *data.get(r + 2, c + 2) == word[2]
                            && *data.get(r + 3, c + 3) == word[3]
                        {
                            count += 1;
                        }
                    }
                    if r >= 3 {
                        // up-right
                        if *data.get(r - 1, c + 1) == word[1]
                            && *data.get(r - 2, c + 2) == word[2]
                            && *data.get(r - 3, c + 3) == word[3]
                        {
                            count += 1;
                        }
                    }
                } else if r + 3 < data.height {
                    // not right
                    // down
                    if *data.get(r + 1, c) == word[1]
                        && *data.get(r + 2, c) == word[2]
                        && *data.get(r + 3, c) == word[3]
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn part2(input: String) -> usize {
    let data = preprocess(input);
    let mut count: usize = 0;
    for r in 1..data.height - 1 {
        for c in 1..data.width - 1 {
            if *data.get(r, c) == b'A'
                && ((*data.get(r - 1, c - 1) == b'M' && *data.get(r + 1, c + 1) == b'S')
                    || (*data.get(r - 1, c - 1) == b'S' && *data.get(r + 1, c + 1) == b'M'))
                && ((*data.get(r - 1, c + 1) == b'M' && *data.get(r + 1, c - 1) == b'S')
                    || (*data.get(r - 1, c + 1) == b'S' && *data.get(r + 1, c - 1) == b'M'))
            {
                count += 1;
            }
        }
    }
    count
}

pub fn test_input() -> String {
    load_file("res/day_04_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_04_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 18;
    static PART1_EXPECTED_RESULT: usize = 2599;
    static PART2_TEST_EXPECTED_RESULT: usize = 9;
    static PART2_EXPECTED_RESULT: usize = 1948;

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
