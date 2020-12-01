#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines()
        .map(|l| {
            let s = String::from(l.trim());
            if let Ok(n) = s.parse::<u32>() {
                n
            } else {
                0
            }
        }).collect()
}

#[aoc(day1, part1, loop)]
pub fn part1(input: &Vec<u32>) -> u32 {
    for a in input {
        for b in input {
            if a+b == 2020 {
                return a*b;
            }
        }
    }
    0
}

#[aoc(day1, part2, loop)]
pub fn part2(input: &Vec<u32>) -> u32 {
    for a in input {
        for b in input {
            for c in input {
                if a+b+c == 2020 {
                    return a*b*c;
                }
            }
        }
    }
    0
}

use itertools::Itertools;

#[aoc(day1, part1, iter)]
pub fn part1_iter(input: &Vec<u32>) -> u64 {
    let mut result = 0;
    for pair in input.iter().combinations(2) {
        let literal = pair.into_iter().map(|x| *x as u64);
        let sum: u64 = literal.clone().sum();
        if sum == 2020 {
            result = literal.clone().product();
            break;
        }
    }
    result
}

#[aoc(day1, part2, iter)]
pub fn part2_iter(input: &Vec<u32>) -> u64 {
    let mut result = 0;
    for pair in input.iter().combinations(3) {
        let literal = pair.into_iter().map(|x| *x as u64);
        let sum: u64 = literal.clone().sum();
        if sum == 2020 {
            result = literal.clone().product();
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let example = "1721\r\n
                979\r\n
                366\r\n
                299\r\n
                675\r\n
                1456";
        assert_eq!(part1(&input_generator(example)), 100);
    }
}