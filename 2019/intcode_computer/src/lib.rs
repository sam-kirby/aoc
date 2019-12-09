//! # Intcode Computer AOC2019
//! Implementation of an intcode computer as defined in the Advent of Code 2019
//! Used in problems 2, 5 and 9 thus far.
//!
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
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
            ExecutionState::Running => {
                write!(f, "Running")
            }
            ExecutionState::Halted(reason) => {
                write!(f, "Halted due to {}", reason)
            }
        }
    }
}

/// ## Machine
/// An intcode computer
/// ### Example
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

        let initial_memory = prog_string.split(",")
            .filter_map(|op_str| op_str.trim().parse::<isize>().ok()).collect::<Vec<isize>>();

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

        let op_code = self.memory[self.inst_pointer];

        match op_code {
            // 99 terminates the program; there are no arguments
            99 => self.exec_state = ExecutionState::Halted(String::from("end of program")),
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
            _ => self.exec_state = ExecutionState::Halted(String::from("unknown opcode")),
        }
    }

    /// Execute the program in memory
    pub fn execute(&mut self) {
        loop {
            self.step();

            match self.exec_state {
                ExecutionState::Halted(_) => {
                    break;
                }
                _ => {}
            }
        }
    }

    /// Get the current execution state of the machine
    pub fn get_exec_state(&self) -> &ExecutionState {
        &self.exec_state
    }

    /// Get the result of the program (memory location 0)
    pub fn get_result(&self) -> isize {
        self.memory[0]
    }
}
