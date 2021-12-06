use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let path = Path::new("input1.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read the input\n{}", why.description()),
    };

    let total_fuel: usize = BufReader::new(file)
        .lines()
        .filter_map(|arg| arg.unwrap().parse::<usize>().ok())
        .fold(0, |sum, mass| sum + mass / 3 - 2);

    println!("Fuel required is: {}", total_fuel);

    Ok(())
}
