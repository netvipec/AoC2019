use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut pixels_info : Vec<u8> = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        pixels_info = line.as_bytes().to_vec();
        break;
    }

    let pi : Vec<u8> = pixels_info.iter().map(|&x| x - ('0' as u8)).collect();

    const ROW : usize = 25;
    const COL : usize = 6;
    const LAYER_SIZE : usize = ROW * COL;

    let mut min_zeros = LAYER_SIZE + 1;
    let mut min_zeros_idx = (pi.len() / LAYER_SIZE) + 1;
    for i in 0..(pi.len() / LAYER_SIZE) {
        let count_zero = pi.iter().skip(i * LAYER_SIZE).take(LAYER_SIZE).filter(|&n| *n == 0).count();
        if count_zero < min_zeros {
            min_zeros = count_zero;
            min_zeros_idx = i;
        }
    }

    let count_ones = pi.iter().skip(min_zeros_idx * LAYER_SIZE).take(LAYER_SIZE).filter(|&n| *n == 1).count();
    let count_twos = pi.iter().skip(min_zeros_idx * LAYER_SIZE).take(LAYER_SIZE).filter(|&n| *n == 2).count();

    println!("Min Zeros: {}, layer idx: {}", min_zeros, min_zeros_idx);
    println!("Count 1: {}", count_ones);
    println!("Count 2: {}", count_twos);
    println!("Solution: {}", count_ones * count_twos);
}
