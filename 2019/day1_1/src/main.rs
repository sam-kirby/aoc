use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn fuel_required(mass: &usize) -> usize {
    mass / 3 - 2
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("input1.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read the input\n{}", why.description()),
    };

    let lines = BufReader::new(file).lines();

    let mut total_fuel: usize = 0;

    for line in lines {
        let mass = line?.parse::<usize>().unwrap();
        total_fuel += fuel_required(&mass);
    }

    println!("Fuel required is: {}", total_fuel);

    Ok(())
}
