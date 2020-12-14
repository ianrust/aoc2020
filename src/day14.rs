use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeLine {
    mask: bool,
    address: u64,
    value: u64,
}

impl CodeLine {
    pub fn new() -> CodeLine {
        CodeLine {
            mask: false,
            address: u64::MAX,
            value: u64::MAX,
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<CodeLine> {
    let bracket_grabber = Regex::new(r"\[(.*)\]").unwrap();
    input
        .lines()
        .map(|l| {
            let mut assign_items = l.trim().split(" = ");
            let lhs = assign_items.next().expect("bad parse of lhs").trim();
            let rhs = assign_items.next().expect("bad parse of lhs").trim();
            if lhs.eq("mask") {
                // (address, value)
                let (a, v) = rhs.chars().fold((0u64, 0u64), |values, bit| match bit {
                    'X' => (values.0 << 1, values.1 << 1),
                    '1' => ((values.0 + 1) << 1, (values.1 + 1) << 1),
                    '0' => ((values.0 + 1) << 1, values.1 << 1),
                    _ => panic!("unrecognized bit char"),
                });
                CodeLine {
                    mask: true,
                    address: a >> 1, // specifies bits to change in this case
                    value: v >> 1,   // specifies what to change them to
                }
            } else {
                CodeLine {
                    mask: false,
                    address: bracket_grabber
                        .captures(lhs)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap(), // parse the address
                    value: rhs.parse::<u64>().unwrap(), // specifies what to change them to
                }
            }
        })
        .collect::<Vec<CodeLine>>()
}

fn mask_set(value: u64, mask_line: &CodeLine) -> u64 {
    let inverse_address = !mask_line.address;
    (inverse_address & value) + mask_line.value
}

fn mask_address(address: u64, mask_line: &CodeLine) -> Vec<usize> {
    let base_address = (address & mask_line.address) | mask_line.value;
    let unset_bits = !mask_line.address;
    let mut addresses = vec![base_address as usize];
    for bit_index in 0..36 {
        // this means this bit is set
        let single_bit_number =
         1 << bit_index;
        if unset_bits & single_bit_number == single_bit_number {
            addresses.extend(
                addresses
                    .iter()
                    .map(|a| a + single_bit_number as usize)
                    .collect::<Vec<usize>>(),
            );
        }
    }
    addresses
}

fn interpret_code(
    code: &Vec<CodeLine>,
    memset: fn(&mut HashMap<usize, u64>, &CodeLine, &CodeLine) -> (),
) -> u64 {
    let mut mem = HashMap::<usize, u64>::new();
    let mut mask_line: &CodeLine = &CodeLine::new();
    for line in code {
        if line.mask {
            mask_line = line;
        } else {
            memset(&mut mem, line, mask_line);
        }
    }
    mem.iter().fold(0, |sum, (_, value)| sum + value)
}

#[aoc(day14, part1)]
pub fn part1(code: &Vec<CodeLine>) -> u64 {
    fn memset_value(mem: &mut HashMap<usize, u64>, line: &CodeLine, mask_line: &CodeLine) {
        mem.remove(&(line.address as usize));
        mem.insert(line.address as usize, mask_set(line.value, mask_line));
    }
    interpret_code(code, memset_value)
}

#[aoc(day14, part2)]
pub fn part2(code: &Vec<CodeLine>) -> u64 {
    fn memset_address(mem: &mut HashMap<usize, u64>, line: &CodeLine, mask_line: &CodeLine) {
        for address in mask_address(line.address, mask_line) {
            mem.remove(&address);
            mem.insert(address, line.value);
        }
    }
    interpret_code(code, memset_address)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                            mem[8] = 11
                            mem[7] = 101
                            mem[8] = 0";

    const SAMPLE2: &str = "mask = 000000000000000000000000000000X1001X
                            mem[42] = 100
                            mask = 00000000000000000000000000000000X0XX
                            mem[26] = 1";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE1)), 165);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE2)), 208);
    }
}
