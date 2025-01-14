use super::load_file::load_file;

type V2 = [f64; 2];
type M2x2 = [[f64; 2]; 2];

fn inverse(m: &M2x2) -> Option<M2x2> {
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    if det == 0.0 {
        None
    } else {
        let det_recip = det.recip();
        Some([
            [m[1][1] * det_recip, m[0][1] * -1.0 * det_recip],
            [m[1][0] * -1.0 * det_recip, m[0][0] * det_recip],
        ])
    }
}

fn mul(m: &M2x2, v: &V2) -> V2 {
    [
        m[0][0] * v[0] + m[0][1] * v[1],
        m[1][0] * v[0] + m[1][1] * v[1],
    ]
}

fn solve(a: &V2, b: &V2, g: &V2) -> Option<usize> {
    let m: M2x2 = [[a[0], b[0]], [a[1], b[1]]];
    if let Some(mi) = inverse(&m) {
        let n = mul(&mi, &g).map(|x| x.round());
        if n[0] * a[0] + n[1] * b[0] == g[0] && n[0] * a[1] + n[1] * b[1] == g[1] {
            Some(n[0] as usize * 3 + n[1] as usize)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug)]
struct Machine {
    prize: V2,
    a: V2,
    b: V2,
}

fn preprocess(input: String) -> Vec<Machine> {
    let machines_strings = input.split("\n\n");
    let count = machines_strings.clone().count();
    let mut result = Vec::<Machine>::with_capacity(count);
    for machine_string in machines_strings {
        let mut lines = machine_string.lines().filter(|x| !x.is_empty()).map(|x| {
            let mut parts = x
                .split(":")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|x| x.trim()[2..].parse::<f64>().unwrap());
            let x: f64 = parts.next().unwrap();
            let y: f64 = parts.next().unwrap();
            assert!(parts.next().is_none());
            [x, y]
        });
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        result.push(Machine { a, b, prize })
    }
    result
}

pub fn part1(input: String) -> usize {
    let machines = preprocess(input);
    let mut result: usize = 0;
    for machine in &machines {
        if let Some(res) = solve(&machine.a, &machine.b, &machine.prize) {
            result += res;
        }
    }
    result
}

pub fn part2(input: String) -> usize {
    let machines = preprocess(input);
    let mut result: usize = 0;
    for machine in &machines {
        if let Some(res) = solve(
            &machine.a,
            &machine.b,
            &machine.prize.map(|x| x + 10000000000000.0),
        ) {
            result += res;
        }
    }
    result
}

pub fn test_input() -> String {
    load_file("res/day_13_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_13_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 480;
    static PART1_EXPECTED_RESULT: usize = 27105;
    static PART2_EXPECTED_RESULT: usize = 101726882250942;

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
