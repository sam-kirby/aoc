use std::error::Error;
use std::path::Path;

use intcode_computer::Machine;

// Input 1 for part 1, 5 for part 2
fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("input5.txt");
    let mut machine = Machine::from_path(path)?;

    machine.execute();

    Ok(())
}
