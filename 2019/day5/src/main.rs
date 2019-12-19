use std::error::Error;
use std::io::Write;
use std::path::Path;

use intcode_computer::Machine;
use text_io::{read, try_read, try_scan};

// Input 1 for part 1, 5 for part 2
fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("input5.txt");
    let mut machine = Machine::from_path(path)?;

    let input_fn = || {
        print!("> ");
        std::io::stdout().flush().unwrap();
        read!()
    };
    let output_fn = |out| println!("= {}", out);
    machine.execute(&input_fn, &output_fn);

    Ok(())
}
