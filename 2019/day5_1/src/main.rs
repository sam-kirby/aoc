use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use intcode_computer::{ExecutionState, Machine};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("simple.txt");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read the input\n{}", why.description()),
    };

    let mut prog_string = String::new();
    file.read_to_string(&mut prog_string)?;

    let prog_vec = prog_string.split(",")
        .map(|op_str| op_str.trim().parse::<isize>().unwrap()).collect::<Vec<isize>>();


    let mut machine = Machine::new(prog_vec.clone());

    loop {
        machine.execute();

        match machine.get_exec_state() {
            ExecutionState::Halted => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
