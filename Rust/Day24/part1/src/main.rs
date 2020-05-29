use std::fs::File;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn print_eris(eris : u32) {
    let mut power2 = 1;
    for _x in 0..5 {
        for _y in 0..5 {
            if (eris & power2) > 0 {
                print!("#");
            } else {
                print!(".");
            }
            power2 *= 2;
        }
        println!("");
    }
}

fn evolve_eris(eris : u32) -> u32 {
    let mut new_eris : u32 = 0;

    let mut power2 = 1;
    for x in 0..5 {
        for y in 0..5 {
            let mut adjacent_info = 0;
            if x > 0 {
                let new_n = power2 >> 5;
                adjacent_info += if (eris & new_n) > 0 { 1 } else { 0 };
            }
            if x < 4 {
                let new_n = power2 << 5;
                adjacent_info += if (eris & new_n) > 0 { 1 } else { 0 };
            }
            if y > 0 {
                let new_n = power2 >> 1;
                adjacent_info += if (eris & new_n) > 0 { 1 } else { 0 };
            }
            if y < 4 {
                let new_n = power2 << 1;
                adjacent_info += if (eris & new_n) > 0 { 1 } else { 0 };
            }

            if (eris & power2) > 0 {
                new_eris |= if adjacent_info == 1 { power2 } else { 0 };
            } else {
                new_eris |= if adjacent_info == 1 || adjacent_info == 2 { power2 } else { 0 };
            }
            power2 *= 2;
        }
    }

    return new_eris;
}

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut eris : u32 = 0;
    let mut power2 : u32 = 1;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.trim().len() == 0 {
            break;
        }

        for c in line.chars() {
            if c == '#' {
                eris |= power2;
            }

            power2 *= 2;
        }
    }

    let mut prev_eris : HashSet<u32> = HashSet::new();

    prev_eris.insert(eris);
    print_eris(eris);
    println!();

    loop {
        let new_eris = evolve_eris(eris);

        if prev_eris.contains(&new_eris) {
            print_eris(new_eris);
            println!("Solution: {}", new_eris);
            break;
        }

        prev_eris.insert(new_eris);
        // println!();
        // print_eris(new_eris);
        eris = new_eris;
    }
}
