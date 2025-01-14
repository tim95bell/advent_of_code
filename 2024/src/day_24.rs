use super::load_file::load_file;
use rand::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Label {
    Other([u8; 3]),
    X(usize),
    Y(usize),
    Z(usize),
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Label::Other(bytes) => write!(
                f,
                "{}{}{}",
                bytes[0] as char, bytes[1] as char, bytes[2] as char
            ),
            Label::X(n) => write!(f, "x{n}"),
            Label::Y(n) => write!(f, "y{n}"),
            Label::Z(n) => write!(f, "z{n:0width$}", width = 2),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum OpType {
    XOR,
    OR,
    AND,
}

impl OpType {
    fn perform(&self, x: bool, y: bool) -> bool {
        match self {
            OpType::XOR => x ^ y,
            OpType::OR => x || y,
            OpType::AND => x && y,
        }
    }
}

impl std::fmt::Display for OpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpType::XOR => "XOR",
                OpType::OR => "OR",
                OpType::AND => "AND",
            }
        )
    }
}

#[derive(Clone)]
struct Wire {
    label: Label,
    value: Option<bool>,
}

impl Wire {
    fn create(label: Label) -> Self {
        Self { label, value: None }
    }
}

impl std::fmt::Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}",
            self.label,
            if let Some(v) = self.value {
                v.to_string()
            } else {
                "?".to_string()
            }
        )
    }
}

#[derive(Clone)]
struct Op {
    op: OpType,
    input: [Wire; 2],
    output: Label,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.input[0], self.op, self.input[1], self.output
        )
    }
}

fn parse_label(x: &str) -> Label {
    let bytes = x.as_bytes();
    assert!(bytes.len() == 3);
    if bytes[0] == b'x' {
        Label::X(parse_two_digit_number(bytes[1], bytes[2]))
    } else if bytes[0] == b'y' {
        Label::Y(parse_two_digit_number(bytes[1], bytes[2]))
    } else if bytes[0] == b'z' {
        Label::Z(parse_two_digit_number(bytes[1], bytes[2]))
    } else {
        let mut arr: [u8; 3] = [0; 3];
        arr[0] = bytes[0];
        arr[1] = bytes[1];
        arr[2] = bytes[2];
        Label::Other(arr)
    }
}

fn parse_op(x: &str) -> OpType {
    match x {
        "XOR" => OpType::XOR,
        "OR" => OpType::OR,
        "AND" => OpType::AND,
        _ => unreachable!(),
    }
}

fn parse_two_digit_number(a: u8, b: u8) -> usize {
    ((a - b'0') as usize) * 10 + ((b - b'0') as usize)
}

fn apply_output(output_label: Label, output_value: bool, output: &mut usize) {
    if output_value {
        if let Label::Z(n) = output_label {
            *output |= 1 << n;
        }
    }
}

fn give_input(
    ops_with_input: &std::collections::HashMap<Label, Vec<usize>>,
    ops: &mut Vec<Op>,
    output: &mut usize,
    input_label: Label,
    input_value: bool,
) {
    if let Some(ops_indices) = ops_with_input.get(&input_label) {
        for op_index in ops_indices {
            let op = &mut ops[*op_index];

            if op.input[0].value.is_none() && op.input[0].label == input_label {
                op.input[0].value = Some(input_value);
                if let Some(other_value) = op.input[1].value {
                    let result = op.op.perform(input_value, other_value);
                    let output_label = op.output;
                    give_input(ops_with_input, ops, output, output_label, result);
                }
            } else if op.input[1].value.is_none() && op.input[1].label == input_label {
                op.input[1].value = Some(input_value);
                if let Some(other_value) = op.input[0].value {
                    let result = op.op.perform(input_value, other_value);
                    let output_label = op.output;
                    give_input(ops_with_input, ops, output, output_label, result);
                }
            }
        }
    } else {
        assert!(if let Label::Z(_) = input_label {
            true
        } else {
            false
        });
        apply_output(input_label, input_value, output);
    }
}

fn run(
    ops: &mut Vec<Op>,
    ops_with_input: &std::collections::HashMap<Label, Vec<usize>>,
    x: usize,
    y: usize,
) -> usize {
    let mut output: usize = 0;
    reset(ops);
    for n in 0..INPUT_BIT_COUNT {
        give_input(
            ops_with_input,
            ops,
            &mut output,
            Label::X(n),
            ((x >> n) & 1) != 0,
        );
        give_input(
            ops_with_input,
            ops,
            &mut output,
            Label::Y(n),
            ((y >> n) & 1) != 0,
        );
    }
    output
}

const INPUT_BIT_COUNT: usize = 45;
const OUTPUT_BIT_COUNT: usize = INPUT_BIT_COUNT + 1;

fn validate(
    ops: &mut Vec<Op>,
    ops_with_input: &std::collections::HashMap<Label, Vec<usize>>,
    n: usize,
) -> bool {
    let mut rng = rand::thread_rng();
    let input_bit_mask = !0usize >> (64 - INPUT_BIT_COUNT);
    let output_bit_mask = !0usize >> (64 - OUTPUT_BIT_COUNT);
    for _ in 0..1000 {
        let x: usize = rng.gen::<usize>() & input_bit_mask;
        let y: usize = rng.gen::<usize>() & input_bit_mask;
        let expected_result = x + y;
        assert!(expected_result == expected_result & output_bit_mask);
        let result = run(ops, ops_with_input, x, y);
        assert!(result == result & output_bit_mask);
        let diff = result ^ expected_result;
        if diff & (!0 >> (64 - (n + 1))) != 0 {
            return false;
        }
    }
    true
}

fn sub_tree(
    set: &mut Vec<usize>,
    ops: &Vec<Op>,
    op_with_output: &std::collections::HashMap<Label, usize>,
    output: Label,
) {
    if let Some(op_index) = op_with_output.get(&output) {
        set.push(*op_index);
        let op = &ops[*op_index];
        sub_tree(set, ops, op_with_output, op.input[0].label);
        sub_tree(set, ops, op_with_output, op.input[1].label);
    }
}

fn preprocess(
    input: String,
) -> (
    Vec<(Label, bool)>,
    std::collections::HashMap<Label, Vec<usize>>,
    std::collections::HashMap<Label, usize>,
    Vec<Op>,
) {
    let mut parts = input.trim().split("\n\n");
    let wires = parts
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.trim().split(": ");
            let label = parse_label(parts.next().unwrap());
            let value = parts.next().unwrap() == "1";
            assert!(parts.next().is_none());
            (label, value)
        })
        .collect::<Vec<(Label, bool)>>();

    let mut ops_with_input = std::collections::HashMap::<Label, Vec<usize>>::new();
    let mut op_with_output = std::collections::HashMap::<Label, usize>::new();
    let mut ops = Vec::<Op>::new();

    for line in parts.next().unwrap().trim().lines() {
        let mut parts = line.split(" ");
        let input1 = parse_label(parts.next().unwrap());
        let op = parse_op(parts.next().unwrap());
        let input2 = parse_label(parts.next().unwrap());
        let arrow = parts.next().unwrap();
        assert!(arrow == "->");
        let output = parse_label(parts.next().unwrap());
        let index = ops.len();
        ops.push(Op {
            op,
            input: [Wire::create(input1), Wire::create(input2)],
            output,
        });
        op_with_output.insert(output, index);
        ops_with_input.entry(input1).or_default().push(index);
        ops_with_input.entry(input2).or_default().push(index);
    }

    (wires, ops_with_input, op_with_output, ops)
}

pub fn part1(input: String) -> usize {
    let (wires, ops_with_input, _op_with_output, mut ops) = preprocess(input);
    let mut output: usize = 0;

    for wire in wires {
        give_input(&ops_with_input, &mut ops, &mut output, wire.0, wire.1);
    }
    return output;
}

pub fn part2(input: String) -> String {
    let (_wires, ops_with_input, op_with_output, mut ops) = preprocess(input);

    let mut final_result = Vec::<String>::with_capacity(4);
    let mut possible_invalid_set = {
        let mut possible_invalid_set = Vec::<usize>::with_capacity(ops.len());
        let mut i = 0;
        possible_invalid_set.resize_with(ops.len(), || {
            let r = i;
            i += 1;
            r
        });
        possible_invalid_set
    };
    let mut set = Vec::<usize>::with_capacity(5);
    for n in 0..INPUT_BIT_COUNT {
        set.clear();
        sub_tree(&mut set, &ops, &op_with_output, Label::Z(n));
        if validate(&mut ops, &ops_with_input, n) {
            for i in &set {
                if let Ok(index) = possible_invalid_set.binary_search(&i) {
                    possible_invalid_set.remove(index);
                }
            }
        } else {
            let mut result = false;
            'outer: for i in &set {
                for j in &possible_invalid_set {
                    if i != j {
                        let tmp = ops[*i].output;
                        ops[*i].output = ops[*j].output;
                        ops[*j].output = tmp;

                        result = validate(&mut ops, &ops_with_input, n);

                        if result {
                            final_result.push(ops[*i].output.to_string());
                            final_result.push(ops[*j].output.to_string());
                            break 'outer;
                        } else {
                            let tmp = ops[*i].output;
                            ops[*i].output = ops[*j].output;
                            ops[*j].output = tmp;
                        }
                    }
                }
            }
            if result {
                set.clear();
                sub_tree(&mut set, &ops, &op_with_output, Label::Z(n));
                for i in &set {
                    if let Ok(index) = possible_invalid_set.binary_search(&i) {
                        possible_invalid_set.remove(index);
                    }
                }
            }
        }
    }

    if validate(&mut ops, &ops_with_input, OUTPUT_BIT_COUNT) {
        final_result.sort();
    }
    final_result.join(",")
}

fn reset(ops: &mut Vec<Op>) {
    for op in ops {
        op.input[0].value = None;
        op.input[1].value = None;
    }
}

pub fn test_input() -> String {
    load_file("res/day_24_test_input.txt")
}

pub fn input() -> String {
    load_file("res/day_24_input.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    static PART1_TEST_EXPECTED_RESULT: usize = 2024;
    static PART1_EXPECTED_RESULT: usize = 50411513338638;
    static PART2_EXPECTED_RESULT: &str = "gfv,hcm,kfs,tqm,vwr,z06,z11,z16";

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
