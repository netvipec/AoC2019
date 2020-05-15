use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut digits = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.trim().len() == 0 {
            break;
        }
        // Show the line and its number.
        for digit in line.as_bytes().iter() {
            digits.push(*digit - ('0' as u8));
        }
    }

    println!("Digits: {:?}", digits);

    let base_pattern = vec![0, 1, 0, -1];
    let pattern_start_idx = 1;
    let mut old_digits = digits.clone();
    for _phase in 1..=100 {
        let mut new_digits = vec![0; digits.len()];

        for j in 1..=digits.len() {
            let mut pattern_idx = if j == 1 { pattern_start_idx } else { pattern_start_idx - 1 };
            let mut repeat_counter = if j == 1 { 0 } else { 1 };
            let mut digit_value : i64 = 0;
            for k in 0..digits.len() {
                digit_value += base_pattern[pattern_idx] as i64 * old_digits[k] as i64;
                repeat_counter += 1;
                if repeat_counter == j {
                    repeat_counter = 0;
                    pattern_idx += 1;
                    if pattern_idx == base_pattern.len() {
                        pattern_idx = 0;
                    }
                }
            }

            new_digits[j-1] = (digit_value.abs() % 10) as u8;
        }
        // println!("{} -> {:?}", _phase, new_digits);
        old_digits = new_digits;
    }

    println!("{} -> {:?}", 100, old_digits);
}
