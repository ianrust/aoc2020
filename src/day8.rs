use crate::console::*;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .map(|l| {
            let mut op_pair = l.trim().split(" ");
            let instruction: (String, i32);
            if let (Some(op), Some(val)) = (op_pair.next(), op_pair.next()) {
                instruction = (String::from(op), val.parse::<i32>().unwrap());
            } else {
                panic!("encountered badly formed row")
            }
            instruction
        })
        .collect::<Vec<(String, i32)>>()
}

#[aoc(day8, part1)]
pub fn part1(program: &Vec<(String, i32)>) -> i32 {
    match run(program) {
        Ok(_) => panic!("this should not return correctly"),
        Err(state_after_single_loop) => return state_after_single_loop.accumulator,
    };
}

#[aoc(day8, part2)]
pub fn part2(program: &Vec<(String, i32)>) -> i32 {
    match fix_and_run(program) {
        Ok(state_after_single_loop) => return state_after_single_loop.accumulator,
        Err(_) => panic!("this should complete"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = "nop +0
                        acc +1
                        jmp +4
                        acc +3
                        jmp -3
                        acc -99
                        acc +1
                        jmp -4
                        acc +6";
        assert_eq!(part1(&input_generator(sample)), 5);
    }

    #[test]
    fn sample2() {
        let sample = "nop +0
                        acc +1
                        jmp +4
                        acc +3
                        jmp -3
                        acc -99
                        acc +1
                        jmp -4
                        acc +6";
        assert_eq!(part2(&input_generator(sample)), 8);
    }
}
