use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

use intcode_computer::{ExecutionState, Machine};

fn main() -> Result<(), Box<dyn Error>> {
    let target_result = env::args().collect::<Vec<String>>()[1].trim().parse::<isize>()?;

    let path = Path::new("input2.txt");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read the input\n{}", why.description()),
    };

    let mut prog_string = String::new();
    file.read_to_string(&mut prog_string)?;

    let prog_vec = prog_string.split(",")
        .map(|op_str| op_str.trim().parse::<isize>().unwrap()).collect::<Vec<isize>>();

    'outer: for verb in 0 as isize..=99 as isize {
        for noun in 0 as isize..=99 as isize {
            let mut machine = Machine::new_with_noun_verb(prog_vec.clone(), noun, verb);

            loop {
                machine.execute();

                match machine.get_exec_state() {
                    ExecutionState::Halted => {
                        break;
                    }
                    _ => {}
                }
            }

            if machine.get_result() == target_result {
                println!("Got result: {}", machine.get_result());
                println!("Noun: {}, Verb: {}", noun, verb);
                println!("Code: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}
