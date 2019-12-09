use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let path = Path::new("input1.txt");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read the input: {}", why.description()),
    };
    let mut string_buffer = String::new();
    file.read_to_string(&mut string_buffer)
        .expect("Failed to read file!");

    let total_fuel: isize = string_buffer
        .lines()
        .filter_map(|arg| arg.parse::<isize>().ok())
        .fold(0, |mut sum, mut mass| {
            loop {
                let fuel_required = mass / 3 - 2;
                if fuel_required > 0 {
                    sum += fuel_required;
                    mass = fuel_required;
                } else {
                    break;
                }
            }
            sum
        });

    println!("Total fuel required is: {}", total_fuel);

    Ok(())
}
