use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        let mass: i32 = line.parse().unwrap();
        let fuel = mass / 3 - 2;
        sum += fuel;
    }

    println!("Fuel: {}", sum);
}
