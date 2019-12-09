use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use intcode_computer::{ExecutionState, Machine};

fn main() -> Result<(), io::Error> {
    let path = Path::new("input2.txt");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read the input\n{}", why.description()),
    };

    let mut prog_string = String::new();
    file.read_to_string(&mut prog_string)?;

    let prog_vec = prog_string.split(",")
        .filter_map(|op_str| op_str.trim().parse::<isize>().ok()).collect::<Vec<isize>>();

    let mut prog = Machine::new_with_noun_verb(prog_vec, 12, 2);

    loop {
        prog.execute();

        match prog.get_exec_state() {
            ExecutionState::Halted => {
                break;
            }
            _ => {}
        }
    }

    println!("Position 0 contains: {}", prog.get_result());

    Ok(())
}