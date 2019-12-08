use std::fmt;
use std::fmt::{Display, Formatter};

use text_io::{read, try_read, try_scan};

#[derive(PartialEq, Debug)]
pub enum ExecutionState {
    Running,
    Halted,
}

impl Display for ExecutionState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ExecutionState::Running => {
                write!(f, "Running")
            }
            ExecutionState::Halted => {
                write!(f, "Halted")
            }
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    memory: Vec<isize>,
    exec_state: ExecutionState,
    inst_pointer: usize,
}

struct Instruction {

}

impl Machine {
    pub fn new(initial_memory: Vec<isize>) -> Machine {
        Machine {
            memory: initial_memory,
            exec_state: ExecutionState::Running,
            inst_pointer: 0
        }
    }

    pub fn new_with_noun_verb(mut initial_memory: Vec<isize>, noun: isize, verb: isize) -> Machine {
        initial_memory[1] = noun;
        initial_memory[2] = verb;
        Machine {
            memory: initial_memory,
            exec_state: ExecutionState::Running,
            inst_pointer: 0,
        }
    }

    pub fn execute(&mut self) {
        if self.exec_state != ExecutionState::Running {
            panic!("Tried to execute a program that had {}!", self.exec_state);
        }

        let op_code = self.memory[self.inst_pointer];

        match op_code {
            // 99 terminates the program; there are no arguments
            99 => self.exec_state = ExecutionState::Halted,
            // 1 and 2 take 3 arguments; two input addresses and one output address
            // 1 adds the contents of the two input addresses and stores the result at the output address
            // 2 multiplies the contents of the two input addresses and stores the result at the output address
            1 | 2 => {
                let arg1_addr = self.memory[self.inst_pointer + 1] as usize;
                let arg2_addr = self.memory[self.inst_pointer + 2] as usize;
                let output_addr = self.memory[self.inst_pointer + 3] as usize;

                match op_code {
                    1 => { self.memory[output_addr] = self.memory[arg1_addr] + self.memory[arg2_addr]; }
                    2 => { self.memory[output_addr] = self.memory[arg1_addr] * self.memory[arg2_addr]; }
                    _ => { unreachable!() }
                }

                self.inst_pointer += 4;
            }
            // 3 and 4 take 1 argument; a target address
            // 3 waits for user input and stores the input at the address specified
            // 4 outputs the content of the address specified to stdout
            3 | 4 => {
                let target_addr = self.memory[self.inst_pointer + 1] as usize;

                match op_code {
                    3 => { self.memory[target_addr] = read!() }
                    4 => { println!("{}", self.memory[target_addr]); }
                    _ => { unreachable!() }
                }

                self.inst_pointer += 2;
            }
            _ => self.exec_state = ExecutionState::Halted,
        }
    }

    pub fn get_exec_state(&self) -> &ExecutionState {
        &self.exec_state
    }

    pub fn get_result(&self) -> isize {
        self.memory[0]
    }
}
