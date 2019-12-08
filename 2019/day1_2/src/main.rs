use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn fuel_required(mass: &isize) -> isize {
    mass / 3 - 2
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("input1.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read the input\n{}", why.description()),
    };

    let lines = BufReader::new(file).lines();

    let mut total_fuel: isize = 0;

    for line in lines {
        let mass = line?.parse::<isize>().unwrap();
        let mut fuel_for_module = fuel_required(&mass);

        let mut delta = fuel_for_module;
        loop {
            let fuel_for_fuel = fuel_required(&delta);
            if fuel_for_fuel <= 0 {
                break;
            } else {
                delta = fuel_for_fuel;
                fuel_for_module += fuel_for_fuel;
            }
        }

        total_fuel += fuel_for_module;
    }

    println!("Total fuel required is: {}", total_fuel);

    Ok(())
}