use super::load_file::load_file;
use glam::IVec2;

type AntennaKinds = std::collections::hash_map::HashMap<u8, Vec<IVec2>>;

fn preprocess(input: String) -> (i32, i32, AntennaKinds) {
    let mut antenna_kinds = AntennaKinds::new();
    let lines = input.lines().filter(|x| !x.is_empty());
    let height = lines.clone().count() as i32;
    let width = lines.clone().next().unwrap().len() as i32;
    for (r, line) in lines.enumerate() {
        for (c, ch) in line.as_bytes().iter().enumerate() {
            if *ch != b'.' {
                antenna_kinds
                    .entry(*ch)
                    .or_insert(Vec::<IVec2>::new())
                    .push(IVec2::new(c as i32, r as i32));
            }
        }
    }
    (width, height, antenna_kinds)
}

pub fn part1(input: String) -> usize {
    let (width, height, antenna_kinds) = preprocess(input);

    let mut antinodes = std::collections::HashSet::<IVec2>::new();
    for (_, antennas) in antenna_kinds {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];
                let a_to_b = b - a;
                let antinode_1 = b + a_to_b;
                let antinode_2 = a - a_to_b;
                if (0..width).contains(&antinode_1.x) && (0..height).contains(&antinode_1.y) {
                    antinodes.insert(antinode_1);
                }
                if (0..width).contains(&antinode_2.x) && (0..height).contains(&antinode_2.y) {
                    antinodes.insert(antinode_2);
                }
            }
        }
    }
    antinodes.len()
}

fn add_antinodes(
    width: i32,
    height: i32,
    antinodes: &mut std::collections::HashSet<IVec2>,
    start: IVec2,
    add: IVec2,
) {
    let mut pos = start;
    while (0..width).contains(&pos.x) && (0..height).contains(&pos.y) {
        antinodes.insert(pos);
        pos += add;
    }
}

pub fn part2(input: String) -> usize {
    let (width, height, antenna_kinds) = preprocess(input);

    let mut antinodes = std::collections::HashSet::<IVec2>::new();
    for (_, antennas) in antenna_kinds {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];
                let a_to_b = b - a;
                add_antinodes(width, height, &mut antinodes, b, a_to_b);
                add_antinodes(width, height, &mut antinodes, a, a_to_b * -1);
            }
        }
    }
    antinodes.len()
}

pub fn test_input() -> String {
    load_file("res/day_08_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_08_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 14;
    static PART1_EXPECTED_RESULT: usize = 313;
    static PART2_TEST_EXPECTED_RESULT: usize = 34;
    static PART2_EXPECTED_RESULT: usize = 1064;

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
