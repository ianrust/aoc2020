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

#[derive(Debug)]
struct ProgramState {
    accumulator: i32,
    current_instruction: i32,
    visited_instructions: Vec<i32>,
}

fn step(program: &Vec<(String, i32)>, program_state: &mut ProgramState) {
    if let Some(instruction) = program.get(program_state.current_instruction as usize) {
        match instruction.0.as_str() {
            "nop" => {
                program_state.current_instruction += 1;
            }
            "acc" => {
                program_state.accumulator += instruction.1;
                program_state.current_instruction += 1;
            }
            "jmp" => {
                program_state.current_instruction += instruction.1;
            }
            _ => {}
        }
    } else {
        panic!("ran out of bounds of program memory (check a jmp or similar)");
    }
}

fn run_from(
    program: &Vec<(String, i32)>,
    starting_program_state: &ProgramState,
) -> Result<ProgramState, ProgramState> {
    let mut program_state = ProgramState {
        accumulator: starting_program_state.accumulator,
        current_instruction: starting_program_state.current_instruction,
        visited_instructions: starting_program_state.visited_instructions.clone(),
    };
    loop {
        // terminate on infinite loop
        match program_state
            .visited_instructions
            .binary_search(&program_state.current_instruction)
        {
            Ok(_) => return Err(program_state),
            Err(ind) => {
                program_state
                    .visited_instructions
                    .insert(ind, program_state.current_instruction);
            }
        }
        // terminate on reaching final instruction (1 out of program bounds)
        if program_state.current_instruction as usize == program.len() {
            return Ok(program_state);
        }

        step(program, &mut program_state);
    }
}

fn run(program: &Vec<(String, i32)>) -> Result<ProgramState, ProgramState> {
    let program_state = ProgramState {
        accumulator: 0,
        current_instruction: 0,
        visited_instructions: Vec::<i32>::new(),
    };
    run_from(program, &program_state)
}

fn twiddle(program: &mut Vec<(String, i32)>, program_state: &ProgramState) {
    let mut instruction = &mut program[program_state.current_instruction as usize];
    if instruction.0.as_str() == "nop" {
        instruction.0 = String::from("jmp");
    } else if instruction.0.as_str() == "jmp" {
        instruction.0 = String::from("nop");
    }
}

fn fix_and_run(program: &Vec<(String, i32)>) -> Result<ProgramState, ProgramState> {
    let mut program_state = ProgramState {
        accumulator: 0,
        current_instruction: 0,
        visited_instructions: Vec::<i32>::new(),
    };
    let mut modified_program = program.clone();

    // change program at each line (doesn't always change), see if it completes. if not, go to next instruction
    loop {
        twiddle(&mut modified_program, &program_state);

        match run_from(&modified_program, &program_state) {
            Ok(state) => return Ok(state),
            Err(_) => {
                // try again
                modified_program = program.clone();
            }
        }

        step(&modified_program, &mut program_state);
    }
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
