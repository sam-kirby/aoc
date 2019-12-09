use std::error::Error;
use std::path::Path;

use intcode_computer::{Machine};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("simple.txt");
    let mut machine = Machine::from_path(path).unwrap();

    machine.execute();

    Ok(())
}
