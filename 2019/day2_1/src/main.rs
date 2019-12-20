use std::io;
use std::path::Path;

use intcode_computer::Machine;

fn main() -> Result<(), io::Error> {
    let path = Path::new("input2.txt");

    let mut machine = Machine::from_path(path)?;
    machine.set_noun_verb(12, 2);

    let mut input_fn = || 0isize;
    let mut output_fn = |_out| {};
    machine.execute(&mut input_fn, &mut output_fn);

    println!("Position 0 contains: {}", machine.result());

    Ok(())
}
