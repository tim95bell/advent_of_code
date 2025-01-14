use super::load_file::load_file;

struct Mul(i32, i32);

fn parse_str(data: &mut &str, s: &str) -> bool {
    if data.starts_with(s) {
        *data = &data[s.len()..];
        true
    } else {
        false
    }
}

fn parse_char(data: &mut &str, c: char) -> bool {
    if data.starts_with(c) {
        *data = &data[c.len_utf8()..];
        true
    } else {
        false
    }
}

fn parse_num(data: &mut &str) -> Option<i32> {
    let num_str = data
        .chars()
        .take(3)
        .take_while(|x| x.is_numeric())
        .collect::<String>();

    match num_str.parse::<i32>() {
        Ok(x) => {
            *data = &data[num_str.len()..];
            Some(x)
        }
        Err(_) => None,
    }
}

fn parse_mul(data: &mut &str) -> Option<Mul> {
    let mut data_copy = *data;
    if parse_str(&mut data_copy, "mul(") {
        if let Some(x) = parse_num(&mut data_copy) {
            if parse_char(&mut data_copy, ',') {
                if let Some(y) = parse_num(&mut data_copy) {
                    if parse_char(&mut data_copy, ')') {
                        *data = data_copy;
                        return Some(Mul(x, y));
                    }
                }
            }
        }
    }
    return None;
}

pub fn part1(input: String) -> i32 {
    let mut data = input.as_str();
    let mut result: i32 = 0;
    while data.len() > 0 {
        if let Some(Mul(x, y)) = parse_mul(&mut data) {
            result += x * y;
        } else {
            data = &data[1..];
        }
    }
    result
}

pub fn part2(input: String) -> i32 {
    let mut data = input.as_str();
    let mut result: i32 = 0;
    let mut enabled = true;
    while data.len() > 0 {
        if enabled {
            if let Some(Mul(x, y)) = parse_mul(&mut data) {
                result += x * y;
                continue;
            }

            if parse_str(&mut data, "don't()") {
                enabled = false;
                continue;
            }
        } else {
            if parse_str(&mut data, "do()") {
                enabled = true;
                continue;
            }
        }
        data = &data[1..];
    }
    result
}

pub fn part1_test_input() -> String {
    load_file("res/day_03_part1_test_input.txt")
}

pub fn part2_test_input() -> String {
    load_file("res/day_03_part2_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_03_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: i32 = 161;
    static PART1_EXPECTED_RESULT: i32 = 189600467;
    static PART2_TEST_EXPECTED_RESULT: i32 = 48;
    static PART2_EXPECTED_RESULT: i32 = 107069718;

    #[test]
    fn part1_with_test_input() {
        assert_eq!(part1(part1_test_input()), PART1_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input() {
        assert_eq!(part2(part2_test_input()), PART2_TEST_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }
}
