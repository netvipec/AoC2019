use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // let filename = "src/input0";
    let filename = "../part1/src/input";
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

    // println!("Digits: {:?}", digits);

    let offset = digits[0] as usize * 1_000_000 + 
                 digits[1] as usize *   100_000 + 
                 digits[2] as usize *    10_000 + 
                 digits[3] as usize *     1_000 +
                 digits[4] as usize *       100 +
                 digits[5] as usize *        10 +
                 digits[6] as usize;
    let digits_offset = offset % digits.len();
    let total_len = 10_000 * digits.len();

    println!("Offset: {}, Digits len {}, Digits Offset: {}, Total len: {}", offset, digits.len(), digits_offset, total_len);

    let total_repeat = (total_len - offset) / digits.len();
    
    let mut old_digits = Vec::new();
    old_digits.extend_from_slice(&digits[digits_offset..]);

    println!("Old digits: {:?}, Repeat: {}", old_digits, total_repeat);

    for _i in 0..total_repeat {
        old_digits.append(&mut digits.clone());
    }

    println!("New Digits Len: {:?}", old_digits.len());

    for _phase in 1..=100 {
        let mut new_digits = vec![0; old_digits.len()];

        let mut total_sum : i64 = old_digits.iter().map(|&a| a as i64).sum();

        for j in 0..old_digits.len() {
            new_digits[j] = (total_sum.abs() % 10) as u8;
            total_sum -= old_digits[j] as i64;
        }
        // println!("{} -> {:?}", _phase, new_digits);
        old_digits = new_digits;
    }

    

    println!("{} -> {:?}", 100, &old_digits[0..8]);
}
