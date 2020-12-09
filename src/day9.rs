#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.trim().parse::<u64>().expect("Didn't get a number"))
        .collect::<Vec<u64>>()
}

fn find_first_invalid_number(input: &Vec<u64>, buffer: usize) -> Result<u64, String> {
    for i in buffer..input.len() {
        let preample: &[u64] = &input[(i - buffer)..i];
        let current_number: u64 = input[i];
        let mut found = false;
        for candidate in preample {
            if &current_number < candidate {
                continue;
            }

            let partner_needed = current_number - candidate;
            match preample.iter().find(|&&num| num == partner_needed) {
                None => found = true,
                Some(_) => {
                    found = false;
                    break;
                }
            }
        }
        if found {
            return Ok(current_number);
        }
    }
    Err(String::from("Didn't find an invalid number"))
}

fn find_weakness(input: &Vec<u64>, buffer: usize) -> Result<u64, String> {
    let invalid_number = find_first_invalid_number(input, buffer).expect("Didn't find invalid");

    for i in 0..input.len() {
        let mut accum = 0;
        let mut end_index = i + 1;
        let mut beg_iter = input.iter().skip(end_index);
        loop {
            accum += beg_iter.next().expect("Out of bounds summing");

            if accum == invalid_number {
                let sequence = &input[i..(end_index + 1)];
                return Ok(sequence.iter().max().expect("Failed to get max")
                    + sequence.iter().min().expect("Failed to get min"));
            } else if accum > invalid_number {
                break;
            }
            end_index += 1;
        }
    }

    Err(String::from("never found a sequence"))
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    find_first_invalid_number(input, 25).expect("Didn't find invalid")
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    find_weakness(input, 25).expect("Error finding weakness")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn sample1() {
        assert_eq!(
            find_first_invalid_number(&input_generator(SAMPLE), 5).expect("Didn't find an invalid"),
            127
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            find_weakness(&input_generator(SAMPLE), 5).expect("Error finding weakness"),
            62
        );
    }
}
