use crate::console::*;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Program {
    Program::parse(input)
}

#[aoc(day8, part1)]
pub fn part1(program: &Program) -> i32 {
    match program.run() {
        Ok(_) => panic!("this should not return correctly"),
        Err(state_after_single_loop) => return state_after_single_loop.accumulator,
    };
}

#[aoc(day8, part2)]
pub fn part2(program: &Program) -> i32 {
    match program.fix_and_run() {
        Ok(state_after_completion) => return state_after_completion.accumulator,
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
