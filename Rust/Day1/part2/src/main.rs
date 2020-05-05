use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "../part1/src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        let mass: i32 = line.parse().unwrap();
        let mut actual_mass = mass;
        let mut total_fuel = 0;
        loop {
            let fuel = actual_mass / 3 - 2;
            if fuel <= 0 {
                break;
            }
            total_fuel += fuel;            
            actual_mass = fuel;
        }
        sum += total_fuel;
    }

    println!("Fuel: {}", sum);
}
