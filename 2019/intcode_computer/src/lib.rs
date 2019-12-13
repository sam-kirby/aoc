//! # Intcode Computer AOC2019
//! Implementation of an intcode computer as defined in the Advent of Code 2019
//! Used in problems 2, 5 and 9 thus far.
//!
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use text_io::{read, try_read, try_scan};

/// ## ExecutionState
/// The possible states the computer can be in
/// The computer is running until it reaches op code 99 or hits an error
/// `ExecutionState::Halted` contains a reason for halting.
#[derive(PartialEq, Debug, Clone)]
pub enum ExecutionState {
    /// The computer is able to execute
    Running,
    /// The computer has halted for the reason given
    Halted(String),
}

impl Display for ExecutionState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ExecutionState::Running => write!(f, "running"),
            ExecutionState::Halted(reason) => write!(f, "halted due to {}", reason),
        }
    }
}

/// ## Machine
/// An intcode computer
/// ### Examples
/// ```
/// use intcode_computer::Machine;
///
/// let mut machine = Machine::new(vec![1, 5, 6, 0, 99, 25, 17]);
/// machine.execute();
/// assert_eq!(42, machine.get_result());
/// ```
#[derive(Debug, Clone)]
pub struct Machine {
    memory: Vec<isize>,
    exec_state: ExecutionState,
    inst_pointer: usize,
}

impl Machine {
    /// Create a new `Machine` with a given initial memory vector
    pub fn new(initial_memory: Vec<isize>) -> Machine {
        Machine {
            memory: initial_memory,
            exec_state: ExecutionState::Running,
            inst_pointer: 0,
        }
    }

    /// Create a new `Machine` using the contents of a file as the initial memory from a
    /// supplied `&Path`
    pub fn from_path(path: &Path) -> Result<Machine, io::Error> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't read the input\n{}", why.description()),
        };

        let mut prog_string = String::new();
        file.read_to_string(&mut prog_string)?;

        let initial_memory = prog_string
            .split(',')
            .filter_map(|op_str| op_str.trim().parse::<isize>().ok())
            .collect::<Vec<isize>>();

        Ok(Machine::new(initial_memory))
    }

    /// Set the noun and verb in the program memory (positions 1 and 2)
    pub fn set_noun_verb(&mut self, noun: isize, verb: isize) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    fn step(&mut self) {
        if self.exec_state != ExecutionState::Running {
            panic!("Tried to execute a program that had {}!", self.exec_state);
        }

        let op_code = self.memory[self.inst_pointer] % 100;
        let access_flags = (self.memory[self.inst_pointer] / 100) as usize;

        match op_code {
            // 99 terminates the program; there are no arguments
            99 => {
                self.exec_state = ExecutionState::Halted(String::from("end of program"));
                self.inst_pointer += 1;
            }
            // 1, 2, 7 and 8 take 3 arguments; two inputs and one output address
            // 1 adds the contents of the two inputs and stores the result at the output address
            // 2 multiplies the contents of the two inputs and stores the result at the output address
            // 7 compares input 1 to input 2. If input 1 is less than input 2, 1 is stored in the output address else 0 is stored
            // 8 compares input 1 to input 2. If they are equal, 1 is stored in the output address else 0 is stored
            1 | 2 | 7 | 8 => {
                let arg0 = self.parse_argument(0, access_flags);
                let arg1 = self.parse_argument(1, access_flags);

                let output_addr = self.memory[self.inst_pointer + 3] as usize;

                self.memory[output_addr] = match op_code {
                    1 => arg0 + arg1,
                    2 => arg0 * arg1,
                    7 => if arg0 < arg1 { 1 } else { 0 },
                    8 => if arg0 == arg1 { 1 } else { 0 },
                    _ => unreachable!(),
                };

                self.inst_pointer += 4;
            }
            // 3 and 4 take 1 argument; a target address
            // 3 waits for user input and stores the input at the address specified
            // 4 outputs to stdout based on the value of its argument
            3 | 4 => {
                match op_code {
                    3 => {
                        print!("> ");
                        std::io::stdout().flush().unwrap();
                        let target_addr = self.memory[self.inst_pointer + 1] as usize;
                        self.memory[target_addr] = read!();
                    }
                    4 => {
                        println!("= {}", self.parse_argument(0, access_flags));
                    }
                    _ => unreachable!(),
                }

                self.inst_pointer += 2;
            }
            // 5 and 6 take 2 arguments; both are inputs
            // 5 checks if the first input is not 0. If this is true, it jumps to the location specified by the second input
            // 6 checks if the first input is 0. If this is true, it jumps to the location specified by the second input
            5 | 6 => {
                let arg0 = self.parse_argument(0, access_flags);
                let do_jump = match op_code {
                    5 => arg0 != 0,
                    6 => arg0 == 0,
                    _ => unreachable!(),
                };

                if do_jump {
                    self.inst_pointer = self.parse_argument(1, access_flags) as usize;
                } else {
                    self.inst_pointer += 3;
                }
            }
            _ => self.exec_state = ExecutionState::Halted(String::from("unknown opcode")),
        }
    }

    fn parse_argument(&self, arg_number: usize, access_flag: usize) -> isize {
        let access_mode = if arg_number == 0 {
            access_flag % 10
        } else {
            access_flag / (10 * arg_number) % 10
        };

        let pointer = self.inst_pointer + arg_number + 1;

        match access_mode {
            // Position mode
            0 => self.memory[self.memory[pointer] as usize],
            // Immediate mode
            1 => self.memory[pointer],
            _ => panic!("Unknown access mode specified"),
        }
    }

    /// Execute the program in memory
    pub fn execute(&mut self) {
        loop {
            self.step();

            if let ExecutionState::Halted(_) = self.exec_state {
                break;
            }
        }
    }

    /// Get the current `ExecutionState` of the machine
    pub fn get_exec_state(&self) -> &ExecutionState {
        &self.exec_state
    }

    /// Get the result of the program (memory location 0)
    pub fn get_result(&self) -> isize {
        self.memory[0]
    }
}
