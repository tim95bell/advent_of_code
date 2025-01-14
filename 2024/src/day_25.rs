use super::load_file::load_file;

const WIDTH: u8 = 5;
const HEIGHT: u8 = 7;

pub fn part1(input: String) -> usize {
    let mut locks = Vec::<[u8; WIDTH as usize]>::new();
    let mut keys = Vec::<[u8; WIDTH as usize]>::new();
    for item in input.trim().split("\n\n") {
        let lines = item.lines();
        let is_lock = lines
            .clone()
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .all(|x| *x == b'#');
        let mut vs: [u8; WIDTH as usize] = [0; WIDTH as usize];
        assert!(lines.clone().count() == (HEIGHT as usize));
        for line in lines {
            let bytes = line.as_bytes();
            assert!(bytes.len() == (WIDTH as usize));
            for i in 0..WIDTH {
                if bytes[i as usize] == b'#' {
                    vs[i as usize] += 1;
                }
            }
        }
        if is_lock {
            locks.push(vs);
        } else {
            keys.push(vs);
        }
    }

    let mut result: usize = 0;
    for lock in &locks {
        'keys: for key in &keys {
            for i in 0..WIDTH {
                if lock[i as usize] + key[i as usize] > HEIGHT {
                    continue 'keys;
                }
            }
            result += 1;
        }
    }
    result
}

pub fn test_input() -> String {
    load_file("res/day_25_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_25_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 3;
    static PART1_EXPECTED_RESULT: usize = 2824;

    #[test]
    fn part1_with_test_input() {
        assert_eq!(part1(test_input()), PART1_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }
}
