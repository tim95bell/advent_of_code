use super::load_file::load_file;

fn to_num(x: &[u8]) -> u16 {
    assert!(x.len() == 2);
    ((x[0] as u16) << 8) | (x[1] as u16)
}

fn tripple(a: u16, b: u16, c: u16) -> u64 {
    let mut xs = [a, b, c];
    xs.sort();
    ((xs[0] as u64) << 32) | ((xs[1] as u64) << 16) | (xs[2] as u64)
}

pub fn part1(input: String) -> usize {
    let mut connections = std::collections::HashMap::<u16, std::collections::HashSet<u16>>::new();
    let mut ts = std::collections::HashSet::<u16>::new();
    for line in input.trim().lines() {
        let mut parts = line.split("-");
        let a_bytes = parts.next().unwrap().as_bytes();
        let a = to_num(a_bytes);
        if a_bytes[0] == b't' {
            ts.insert(a);
        }
        let b_bytes = parts.next().unwrap().as_bytes();
        let b = to_num(b_bytes);
        if b_bytes[0] == b't' {
            ts.insert(b);
        }
        assert!(parts.next().is_none());
        connections
            .entry(a)
            .or_insert(std::collections::HashSet::<u16>::new())
            .insert(b);
        connections
            .entry(b)
            .or_insert(std::collections::HashSet::<u16>::new())
            .insert(a);
    }

    let mut results = std::collections::HashSet::<u64>::new();
    for t in ts {
        for c in connections.get(&t).unwrap() {
            // t-c, look at all of c's connections, if any of them are connected to t, then its a tripple

            for c_connection in connections.get(c).unwrap() {
                if connections.get(c_connection).unwrap().contains(&t) {
                    let x = tripple(t, *c, *c_connection);
                    results.insert(x);
                }
            }
        }
    }
    results.len()
}

pub fn part2(input: String) -> String {
    let mut connections = std::collections::HashMap::<u16, std::collections::HashSet<u16>>::new();
    for line in input.trim().lines() {
        let mut parts = line.split("-");
        let a_bytes = parts.next().unwrap().as_bytes();
        let a = to_num(a_bytes);
        let b_bytes = parts.next().unwrap().as_bytes();
        let b = to_num(b_bytes);
        assert!(parts.next().is_none());
        connections
            .entry(a)
            .or_insert(std::collections::HashSet::<u16>::new())
            .insert(b);
        connections
            .entry(b)
            .or_insert(std::collections::HashSet::<u16>::new())
            .insert(a);
    }

    let mut groups = Vec::<std::collections::HashSet<u16>>::new();

    for (a, a_connections) in connections {
        let mut new_groups = Vec::<std::collections::HashSet<u16>>::new();
        for group in &mut groups {
            if group.is_subset(&a_connections) {
                new_groups.push(group.clone());
                new_groups.last_mut().unwrap().insert(a);
            }
        }
        groups.append(&mut new_groups);
        groups.push(std::collections::HashSet::<u16>::new());
        groups.last_mut().unwrap().insert(a);
    }

    groups.sort_by(|a, b| a.len().cmp(&b.len()));
    let biggest_group = groups.last().unwrap();
    let mut result = Vec::<[char; 2]>::with_capacity(biggest_group.len());
    for x in biggest_group {
        let mut chs: [char; 2] = [' '; 2];
        chs[0] = (*x >> 8) as u8 as char;
        chs[1] = *x as u8 as char;
        result.push(chs);
    }
    result.sort();

    let mut string_result = String::new();
    for i in 0..result.len() {
        let s: String = result[i].iter().collect();
        string_result += &s;
        if i != result.len() - 1 {
            string_result += ",";
        }
    }

    string_result
}

pub fn test_input() -> String {
    load_file("res/day_23_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_23_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 7;
    static PART1_EXPECTED_RESULT: usize = 1173;
    static PART2_TEST_EXPECTED_RESULT: &str = "co,de,ka,ta";
    static PART2_EXPECTED_RESULT: &str = "cm,de,ez,gv,hg,iy,or,pw,qu,rs,sn,uc,wq";

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
