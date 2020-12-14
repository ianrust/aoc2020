use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeLine {
    mask: bool,
    address: u128,
    value: u128,
}

impl CodeLine {
    pub fn new() -> CodeLine {
        CodeLine {
            mask: false,
            address: u128::MAX,
            value: u128::MAX,
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
                let (a, v) = rhs.chars().fold((0u128, 0u128), |values, bit| match bit {
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
                        .parse::<u128>()
                        .unwrap(), // parse the address
                    value: rhs.parse::<u128>().unwrap(), // specifies what to change them to
                }
            }
        })
        .collect::<Vec<CodeLine>>()
}

fn mask_set(value: u128, mask_line: &CodeLine) -> u128 {
    let inverse_address = !mask_line.address;
    (inverse_address & value) + mask_line.value
}

fn mask_address(address: u128, mask_line: &CodeLine) -> Vec<usize> {
    let base_address = (address & mask_line.address) | mask_line.value;
    let unset_bits = !mask_line.address;
    let mut components = Vec::<u128>::new();
    for bit_index in 0..36 {
        // this means this bit is set
        let single_bit_number = 1 << bit_index;
        if unset_bits & single_bit_number == single_bit_number {
            components.push(single_bit_number);
        }
    }

    let mut addresses = vec![base_address as usize];
    // get all numbers to add to base address
    for comb_len in 1..(components.len() + 1) {
        for comb in components.iter().combinations(comb_len) {
            let comb_nums = comb.iter().map(|x| **x as u128);
            addresses.push(comb_nums.sum::<u128>() as usize + base_address as usize);
        }
    }

    addresses
}

#[aoc(day14, part1)]
pub fn part1(code: &Vec<CodeLine>) -> u128 {
    let mut mem = HashMap::<usize, u128>::new();
    let mut mask_line: &CodeLine = &CodeLine::new();
    for line in code {
        if line.mask {
            mask_line = line;
        } else {
            mem.remove(&(line.address as usize));
            mem.insert(line.address as usize, mask_set(line.value, mask_line));
        }
    }
    mem.iter().fold(0, |sum, (_, value)| sum + value)
}

#[aoc(day14, part2)]
pub fn part2(code: &Vec<CodeLine>) -> u128 {
    let mut mem = HashMap::<usize, u128>::new();
    let mut mask_line: &CodeLine = &CodeLine::new();
    for line in code {
        if line.mask {
            mask_line = line;
        } else {
            for address in mask_address(line.address, mask_line) {
                mem.remove(&address);
                mem.insert(address, line.value);
            }
        }
    }

    mem.iter().fold(0, |sum, (_, value)| sum + value)
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
