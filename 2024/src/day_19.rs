use super::load_file::load_file;

#[derive(Clone, Copy)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

const COLORS: [Color; 5] = [
    Color::White,
    Color::Blue,
    Color::Black,
    Color::Red,
    Color::Green,
];

fn ascii_to_color(ch: u8) -> Color {
    match ch {
        b'w' => Color::White,
        b'u' => Color::Blue,
        b'b' => Color::Black,
        b'r' => Color::Red,
        b'g' => Color::Green,
        _ => {
            unreachable!()
        }
    }
}

struct Trie {
    children: [Option<Box<Trie>>; COLORS.len()],
    end: bool,
    depth: usize,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: [const { None }; COLORS.len()],
            end: false,
            depth: 0,
        }
    }

    fn get(&self, color: Color) -> Option<&Trie> {
        if let Some(child) = &self.children[color as usize] {
            Some(&child)
        } else {
            None
        }
    }

    fn add(&mut self, color: Color) -> &mut Self {
        if self.children[color as usize].is_none() {
            self.children[color as usize] = Some(Box::new(Self::new()));
            let x = self.children[color as usize].as_mut().unwrap();
            x.depth = self.depth + 1;
            x
        } else {
            self.children[color as usize].as_mut().unwrap()
        }
    }
}

fn match_str(root_trie: &Trie, trie: &Trie, bytes: &[u8]) -> bool {
    assert!(!root_trie.end);

    if bytes.len() == 0 {
        return true;
    }

    if trie.end {
        if match_str(root_trie, root_trie, bytes) {
            return true;
        }
    }

    let color = ascii_to_color(bytes[0]);
    if let Some(new_trie) = trie.get(color) {
        if match_str(root_trie, new_trie, &bytes[1..]) {
            return true;
        }
    }

    false
}

type Cache<'a> = std::collections::HashMap<&'a [u8], usize>;

fn match_strs<'a>(root_trie: &'a Trie, cache: &mut Cache<'a>, bytes: &'a [u8]) -> usize {
    assert!(!root_trie.end);

    if let Some(result) = cache.get(bytes) {
        return *result;
    }

    if bytes.len() == 0 {
        cache.insert(bytes, 0);
        return 0;
    }

    let mut count: usize = 0;

    let mut failed = false;
    let mut trie = root_trie;
    for i in 0..bytes.len() {
        if trie.end {
            count += match_strs(root_trie, cache, &bytes[i..]);
        }

        let color = ascii_to_color(bytes[i]);
        if let Some(new_trie) = trie.get(color) {
            trie = new_trie;
        } else {
            failed = true;
            break;
        }
    }

    if !failed && trie.end {
        count += 1;
    }

    cache.insert(bytes, count);
    count
}

pub fn part1(input: String) -> usize {
    // white (w), blue (u), black (b), red (r), or green (g)
    let mut trie = Trie::new();
    let mut parts = input.split("\n\n");
    for pattern in parts.next().unwrap().trim().split(", ") {
        let mut trie = &mut trie;
        for ch in pattern.as_bytes() {
            trie = trie.add(ascii_to_color(*ch));
        }
        trie.end = true;
    }
    let mut count: usize = 0;
    for line in parts.next().unwrap().trim().lines() {
        if match_str(&trie, &trie, line.as_bytes()) {
            count += 1;
        }
    }
    assert!(parts.next().is_none());
    count
}

pub fn part2(input: String) -> usize {
    let mut trie = Trie::new();
    let mut parts = input.split("\n\n");
    for pattern in parts.next().unwrap().trim().split(", ") {
        let mut trie = &mut trie;
        for ch in pattern.as_bytes() {
            trie = trie.add(ascii_to_color(*ch));
        }
        trie.end = true;
    }

    let mut count: usize = 0;
    let mut cache = Cache::new();
    for line in parts.next().unwrap().trim().lines() {
        count += match_strs(&trie, &mut cache, line.as_bytes());
    }
    assert!(parts.next().is_none());
    count
}

pub fn test_input() -> String {
    load_file("res/day_19_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_19_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 6;
    static PART1_EXPECTED_RESULT: usize = 355;
    static PART2_TEST_EXPECTED_RESULT: usize = 16;
    static PART2_EXPECTED_RESULT: usize = 732978410442050;

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
