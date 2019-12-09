use std::error::Error;

use intcode_computer::Machine;
use std::path::Path;

// Input 1 for part 1, 5 for part 2
fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("input5.txt");
    let mut machine = Machine::from_path(path)?;

    machine.execute();

    Ok(())
}
