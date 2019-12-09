use std::env;
use std::error::Error;
use std::path::Path;

use intcode_computer::Machine;

fn main() -> Result<(), Box<dyn Error>> {
    let target_result = env::args().collect::<Vec<String>>()[1].trim().parse::<isize>()?;

    let path = Path::new("input2.txt");
    let initial_machine = Machine::from_path(path)?;

    'outer: for verb in 0isize..=99isize {
        for noun in 0isize..=99isize {
            let mut machine = initial_machine.clone();
            machine.set_noun_verb(noun, verb);

            machine.execute();

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
