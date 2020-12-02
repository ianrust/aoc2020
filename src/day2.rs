use std::ops::BitXor;

#[derive(Debug, PartialEq)]
struct Scalar(bool);

impl BitXor for Scalar {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

#[derive(Debug)]
pub struct Rule {
    low: usize,
    high: usize,
    letter: char,
}

#[derive(Debug)]
pub struct PasswordCandidate {
    rule: Rule,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordCandidate> {
    input.lines().map( |l| {
        let s = String::from(l.trim());
        let components: Vec<&str> = s.split(" ").collect();
        let range: Vec<&str> = components[0].split("-").collect();
        let cand = PasswordCandidate{
            rule: Rule{
                low: range[0].parse::<usize>().unwrap(),
                high: range[1].parse::<usize>().unwrap(),
                letter: components[1].chars().next().unwrap(),
            },
            password: String::from(components[2]),
        };
        cand
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<PasswordCandidate>) -> u32 {
    let mut count: u32 = 0;
    for cand in input.iter() {
        let num = cand.password.matches(cand.rule.letter).count();
        if num >= cand.rule.low && num <= cand.rule.high {
            count += 1;
        }
    }
    count
}
#[aoc(day2, part2)]
pub fn part2(input: &Vec<PasswordCandidate>) -> u32 {
    let mut count: u32 = 0;
    for cand in input.iter() {
        let char1 = cand.password.chars().nth(cand.rule.low-1).unwrap();
        let char2 = cand.password.chars().nth(cand.rule.high-1).unwrap();
        if (char1 == cand.rule.letter) ^ (char2 == cand.rule.letter) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = "1-3 a: abcde
                      1-3 b: cdefg
                      2-9 c: ccccccccc";
        assert_eq!(part1(&input_generator(sample)), 2);
    }

    #[test]
    fn sample2() {
        let sample = "1-3 a: abcde
                      1-3 b: cdefg
                      2-9 c: ccccccccc";
        assert_eq!(part2(&input_generator(sample)), 1);
    }
}