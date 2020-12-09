#[derive(Debug, Clone)]
pub struct ProgramState {
    pub accumulator: i32,
    pub current_instruction: usize,
    pub visited_instructions: Vec<usize>,
}

impl ProgramState {
    fn new() -> Self {
        ProgramState {
            accumulator: 0,
            current_instruction: 0,
            visited_instructions: Vec::<usize>::new(),
        }
    }
}

#[derive(Clone)]
pub struct Instruction {
    op: String,
    value: i32,
}

#[derive(Clone)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn parse(input: &str) -> Self {
        Program {
            instructions: input
                .lines()
                .map(|l| {
                    let mut op_pair = l.trim().split(" ");
                    if let (Some(op), Some(val)) = (op_pair.next(), op_pair.next()) {
                        Instruction {
                            op: String::from(op),
                            value: val.parse::<i32>().expect("Invalid modifer to parse"),
                        }
                    } else {
                        panic!("encountered badly formed row")
                    }
                })
                .collect::<Vec<Instruction>>(),
        }
    }

    pub fn twiddle(&mut self, program_state: &ProgramState) {
        let instruction = &mut self.instructions[program_state.current_instruction];
        match instruction.op.as_str() {
            "nop" => instruction.op = String::from("jmp"),
            "jmp" => instruction.op = String::from("nop"),
            "acc" => instruction.op = String::from("acc"),
            _ => panic!("Invalid op to twiddle"),
        };
    }

    fn step(&self, program_state: &ProgramState) -> Result<ProgramState, ProgramState> {
        let mut next_program_state = program_state.clone();

        // terminate on infinite loop
        match next_program_state
            .visited_instructions
            .binary_search(&program_state.current_instruction)
        {
            Ok(_) => return Err(next_program_state),
            Err(ind) => {
                next_program_state
                    .visited_instructions
                    .insert(ind, program_state.current_instruction);
            }
        }

        let instruction = self
            .instructions
            .get(program_state.current_instruction)
            .expect("ran out of bounds of program memory (check a jmp or similar)");

        match instruction.op.as_str() {
            "nop" => {
                next_program_state.current_instruction += 1;
            }
            "acc" => {
                next_program_state.accumulator += instruction.value;
                next_program_state.current_instruction += 1;
            }
            "jmp" => {
                if instruction.value > 0 {
                    next_program_state.current_instruction += instruction.value as usize;
                } else {
                    next_program_state.current_instruction -= (-instruction.value) as usize;
                }
            }
            _ => {}
        }

        Ok(next_program_state)
    }

    pub fn run_from(
        &self,
        starting_program_state: &ProgramState,
    ) -> Result<ProgramState, ProgramState> {
        let mut program_state = starting_program_state.clone();
        loop {
            // terminate on reaching final instruction (1 out of program bounds)
            if program_state.current_instruction == self.instructions.len() {
                return Ok(program_state);
            }
            match self.step(&program_state) {
                Ok(state) => program_state = state,
                error => return error,
            }
        }
    }

    pub fn run(&self) -> Result<ProgramState, ProgramState> {
        let program_state = ProgramState::new();
        self.run_from(&program_state)
    }

    pub fn fix_and_run(&self) -> Result<ProgramState, ProgramState> {
        let mut program_state = ProgramState::new();
        let mut modified_program = self.clone();
        // change program at each line (doesn't always change), see if it completes. if not, go to next instruction
        loop {
            modified_program.twiddle(&program_state);
            // run from this point to see if it terminates or infinitely loops
            match modified_program.run_from(&program_state) {
                Ok(state) => return Ok(state),
                Err(_) => {
                    // try again, ie return to normal
                    modified_program.twiddle(&program_state);
                }
            }
            program_state = modified_program
                .step(&program_state)
                .expect("infinitely looped, not fixed");
        }
    }
}
