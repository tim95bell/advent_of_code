use super::load_file::load_file;
use super::matrix::Matrix;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Plant {
    kind: u16,
}

impl Plant {
    fn create(kind: u16) -> Self {
        Plant { kind }
    }
}

impl std::fmt::Display for Plant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = self.kind;
        write!(f, "\t{num}")
    }
}

struct LineData {
    a: usize,
    p: usize,
    e: usize,
    line_and_last_line: [(Vec<usize>, Vec<usize>); 2],
}

impl LineData {
    pub fn get_line(&self, r: usize) -> &(Vec<usize>, Vec<usize>) {
        &self.line_and_last_line[r & 1]
    }

    pub fn get_line_mut(&mut self, r: usize) -> &mut (Vec<usize>, Vec<usize>) {
        &mut self.line_and_last_line[r & 1]
    }

    pub fn get_last_line(&self, r: usize) -> &(Vec<usize>, Vec<usize>) {
        &self.line_and_last_line[r + 1 & 1]
    }

    pub fn get_last_line_mut(&mut self, r: usize) -> &mut (Vec<usize>, Vec<usize>) {
        &mut self.line_and_last_line[r + 1 & 1]
    }

    fn empty() -> Self {
        LineData {
            a: 0,
            p: 0,
            e: 0,
            line_and_last_line: [
                (Vec::<usize>::new(), Vec::<usize>::new()),
                (Vec::<usize>::new(), Vec::<usize>::new()),
            ],
        }
    }
}

fn traverse(
    r: usize,
    c: usize,
    new: Plant,
    old: Plant,
    m: &mut Map,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(r, c)) {
        return;
    }

    let cur = m.get_mut(r, c);
    if *cur != old {
        return;
    }

    *cur = new;
    visited.insert((r, c));

    if c > 0 {
        // can go left
        traverse(r, c - 1, new, old, m, visited);
    }

    if c + 1 < m.width {
        // can go right
        traverse(r, c + 1, new, old, m, visited);
    }

    if r > 0 {
        // can go up
        traverse(r - 1, c, new, old, m, visited);
    }

    if r + 1 < m.height {
        // can go down
        traverse(r + 1, c, new, old, m, visited);
    }
}

fn split_patches(m: &mut Map, start: usize) -> usize {
    let mut count = start;
    let mut visited = HashSet::<(usize, usize)>::new();
    for r in 0..m.height {
        for c in 0..m.width {
            if !visited.contains(&(r, c)) {
                traverse(
                    r,
                    c,
                    Plant::create(count as u16),
                    *m.get(r, c),
                    m,
                    &mut visited,
                );
                count += 1;
            }
        }
    }
    for r in 0..m.height {
        for c in 0..m.width {
            m.get_mut(r, c).kind -= start as u16;
        }
    }
    count - start
}

fn do_line(is_row: bool, r: usize, c: usize, m: &Map, data: &mut Vec<LineData>) {
    let outer_index = if is_row { r } else { c };
    let inner_index = if is_row { c } else { r };
    let cur = *m.get(r, c);
    let first = inner_index == 0;
    if first {
        data[cur.kind as usize]
            .get_line_mut(outer_index)
            .0
            .push(inner_index);
        if is_row {
            data[cur.kind as usize].a += 1;
        }
    } else {
        let prev = if is_row {
            *m.get(r, c - 1)
        } else {
            *m.get(r - 1, c)
        };
        if is_row {
            data[cur.kind as usize].a += 1;
        }
        if prev != cur {
            data[prev.kind as usize]
                .get_line_mut(outer_index)
                .1
                .push(inner_index);
            data[cur.kind as usize]
                .get_line_mut(outer_index)
                .0
                .push(inner_index);
        }
    }
}

fn line_end(is_row: bool, r: usize, c: usize, m: &Map, data: &mut Vec<LineData>) {
    let outer_index = if is_row { r } else { c };
    let inner_index = if is_row { c } else { r };
    let cur = m.get(r, c);
    data[cur.kind as usize]
        .get_line_mut(outer_index)
        .1
        .push(inner_index + 1);
    for i in 0..data.len() {
        assert!(data[i].get_line(outer_index).0.len() == data[i].get_line(outer_index).1.len());
        data[i].p += data[i].get_line(outer_index).0.len() * 2;
        data[i].e += data[i]
            .get_line(outer_index)
            .0
            .iter()
            .filter(|x| !data[i].get_last_line(outer_index).0.contains(x))
            .count();
        data[i].e += data[i]
            .get_line(outer_index)
            .1
            .iter()
            .filter(|x| !data[i].get_last_line(outer_index).1.contains(x))
            .count();
        data[i].get_last_line_mut(outer_index).0.clear();
        data[i].get_last_line_mut(outer_index).1.clear();
    }
}

type Map = Matrix<Plant>;

pub fn part1(input: String) -> usize {
    let mut context = HashMap::<u8, u8>::new();
    let mut m = Matrix::create_from_string_with_context(&input, &mut context, |context, x| {
        let count = context.keys().count() as u8;
        Plant::create(*context.entry(x).or_insert(count) as u16)
    });
    let count = context.keys().count();
    let count = split_patches(&mut m, count);
    let mut row_data = Vec::<LineData>::with_capacity(count);
    row_data.resize_with(count, LineData::empty);

    // rows
    for r in 0..m.height {
        for c in 0..m.width {
            do_line(true, r, c, &m, &mut row_data);
        }
        line_end(true, r, m.width - 1, &m, &mut row_data);
    }

    // clear line cache data
    for line in &mut row_data {
        line.line_and_last_line[0].0.clear();
        line.line_and_last_line[0].1.clear();
        line.line_and_last_line[1].0.clear();
        line.line_and_last_line[1].1.clear();
    }

    // cols
    for c in 0..m.width {
        for r in 0..m.height {
            do_line(false, r, c, &m, &mut row_data);
        }
        line_end(false, m.height - 1, c, &m, &mut row_data);
    }

    let mut sum: usize = 0;
    for i in 0..row_data.len() {
        sum += row_data[i].a * row_data[i].p;
    }
    sum
}

pub fn part2(input: String) -> usize {
    let mut context = HashMap::<u8, u8>::new();
    let mut m = Matrix::create_from_string_with_context(&input, &mut context, |context, x| {
        let count = context.keys().count() as u8;
        Plant::create(*context.entry(x).or_insert(count) as u16)
    });
    let count = context.keys().count();
    let count = split_patches(&mut m, count);
    let mut row_data = Vec::<LineData>::with_capacity(count);
    row_data.resize_with(count, LineData::empty);

    // rows
    for r in 0..m.height {
        for c in 0..m.width {
            do_line(true, r, c, &m, &mut row_data);
        }
        line_end(true, r, m.width - 1, &m, &mut row_data);
    }

    // clear line cache data
    for line in &mut row_data {
        line.line_and_last_line[0].0.clear();
        line.line_and_last_line[0].1.clear();
        line.line_and_last_line[1].0.clear();
        line.line_and_last_line[1].1.clear();
    }

    // cols
    for c in 0..m.width {
        for r in 0..m.height {
            do_line(false, r, c, &m, &mut row_data);
        }
        line_end(false, m.height - 1, c, &m, &mut row_data);
    }

    let mut sum: usize = 0;
    for i in 0..row_data.len() {
        sum += row_data[i].a * row_data[i].e;
    }
    sum
}

pub fn part1_test_input1() -> String {
    load_file("res/day_12_part1_test_input1.txt")
}

pub fn part1_test_input2() -> String {
    load_file("res/day_12_part1_test_input2.txt")
}

pub fn part1_test_input3() -> String {
    load_file("res/day_12_part1_test_input3.txt")
}

pub fn part2_test_input1() -> String {
    load_file("res/day_12_part2_test_input1.txt")
}

pub fn part2_test_input2() -> String {
    load_file("res/day_12_part2_test_input2.txt")
}

pub fn part2_test_input3() -> String {
    load_file("res/day_12_part2_test_input3.txt")
}

pub fn part2_test_input4() -> String {
    load_file("res/day_12_part2_test_input4.txt")
}

pub fn part2_test_input5() -> String {
    load_file("res/day_12_part2_test_input5.txt")
}

pub fn input() -> String {
    load_file("res/day_12_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST1_EXPECTED_RESULT: usize = 140;
    static PART1_TEST2_EXPECTED_RESULT: usize = 772;
    static PART1_TEST3_EXPECTED_RESULT: usize = 1930;
    static PART1_EXPECTED_RESULT: usize = 1465968;
    static PART2_TEST1_EXPECTED_RESULT: usize = 80;
    static PART2_TEST2_EXPECTED_RESULT: usize = 436;
    static PART2_TEST3_EXPECTED_RESULT: usize = 236;
    static PART2_TEST4_EXPECTED_RESULT: usize = 368;
    static PART2_TEST5_EXPECTED_RESULT: usize = 1206;
    static PART2_EXPECTED_RESULT: usize = 897702;

    #[test]
    fn part1_with_test_input1() {
        assert_eq!(part1(part1_test_input1()), PART1_TEST1_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_test_input2() {
        assert_eq!(part1(part1_test_input2()), PART1_TEST2_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_test_input3() {
        assert_eq!(part1(part1_test_input3()), PART1_TEST3_EXPECTED_RESULT);
    }

    #[test]
    fn part1_with_input() {
        assert_eq!(part1(input()), PART1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input1() {
        assert_eq!(part2(part2_test_input1()), PART2_TEST1_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input2() {
        assert_eq!(part2(part2_test_input2()), PART2_TEST2_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input3() {
        assert_eq!(part2(part2_test_input3()), PART2_TEST3_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input4() {
        assert_eq!(part2(part2_test_input4()), PART2_TEST4_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_test_input5() {
        assert_eq!(part2(part2_test_input5()), PART2_TEST5_EXPECTED_RESULT);
    }

    #[test]
    fn part2_with_input() {
        assert_eq!(part2(input()), PART2_EXPECTED_RESULT);
    }
}
