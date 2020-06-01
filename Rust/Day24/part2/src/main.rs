use std::fs::File;
use std::collections::HashMap;
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

fn evolve_eris_one(eris : u32, eris_out : u32, eris_in : u32) -> u32 {
    let mut new_eris : u32 = 0;

    let mut power2 = 1;
    for x in 0..5 {
        for y in 0..5 {
            if x == 2 && y == 2 {
                power2 *= 2;
                continue;
            }

            let mut adjacent_info = 0;
            if x > 0 {
                if x == 3 && y == 2 {
                    for i in 20..25 {
                        adjacent_info += if (eris_in & (1u32 << i)) > 0 { 1 } else { 0 };
                    }
                } else {
                    let new_n = power2 >> 5;
                    adjacent_info += if (eris & new_n) > 0 { 1 } else { 0 };
                }
            } else {
                let new_n = 1u32 << 7;
                adjacent_info += if (eris_out & new_n) > 0 { 1 } else { 0 };
            }

            if x < 4 {
                if x == 1 && y == 2 {
                    for i in 0..5 {
                        adjacent_info += if eris_in & (1u32 << i) > 0 { 1 } else { 0 };
                    }
                } else {
                    let new_n = power2 << 5;
                    adjacent_info += if (eris & new_n) > 0 { 1 } else { 0 };
                }
            } else {
                let new_n = 1u32 << 17;
                adjacent_info += if (eris_out & new_n) > 0 { 1 } else { 0 };
            }

            if y > 0 {
                if x == 2 && y == 3 {
                    for i in 0..5 {
                        adjacent_info += if (eris_in & (1u32 << (5*i + 4))) > 0 { 1 } else { 0 };
                    }
                } else {
                    let new_n = power2 >> 1;
                    adjacent_info += if (eris & new_n) > 0 { 1 } else { 0 };
                }
            } else {
                let new_n = 1u32 << 11;
                adjacent_info += if (eris_out & new_n) > 0 { 1 } else { 0 };
            }

            if y < 4 {
                if x == 2 && y == 1 {
                    for i in 0..5 {
                        adjacent_info += if (eris_in & (1u32 << (5*i))) > 0 { 1 } else { 0 };
                    }
                } else {
                    let new_n = power2 << 1;
                    adjacent_info += if (eris & new_n) > 0 { 1 } else { 0 };
                }
            } else {
                let new_n = 1u32 << 13;
                adjacent_info += if (eris_out & new_n) > 0 { 1 } else { 0 };
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

fn evolve_eris(eris_levels : &HashMap<i32, u32>) -> HashMap<i32, u32> {
    let mut level : i32 = 0;
    let mut new_eris_levels : HashMap<i32, u32> = HashMap::new();

    loop {
        if !eris_levels.contains_key(&level) && !eris_levels.contains_key(&(level - 1)) {
            break;
        }

        let eris_0 = match eris_levels.get(&level) {
            Some(x) => *x,
            None => 0
        };
        let eris_out = match eris_levels.get(&(level - 1)) {
            Some(x) => *x,
            None => 0
        };
        let eris_in = match eris_levels.get(&(level + 1)) {
            Some(x) => *x,
            None => 0
        };
        let new_eris = evolve_eris_one(eris_0, eris_out, eris_in);
        new_eris_levels.insert(level, new_eris);

        if level > 0 {
            let n_level = -level;
            let eris_n0 = match eris_levels.get(&n_level) {
                Some(x) => *x,
                None => 0
            };
            let eris_nout = match eris_levels.get(&(n_level - 1)) {
                Some(x) => *x,
                None => 0
            };
            let eris_nin = match eris_levels.get(&(n_level + 1)) {
                Some(x) => *x,
                None => 0
            };

            let new_neris = evolve_eris_one(eris_n0, eris_nout, eris_nin);
            new_eris_levels.insert(n_level, new_neris);
        }

        level += 1;
    }

    return new_eris_levels;
}

fn count_bugs(eris : u32) -> u32 {
    let mut counter :u32 = 0;
    for i in 0..25 {
        counter += if (eris & (1 << i)) > 0 { 1 } else { 0 };
    }
    return counter;
}

fn main() {
    let filename = "../part1/src/input";
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

    let mut eris_levels : HashMap<i32, u32> = HashMap::new();

    eris_levels.insert(0, eris);
    
    for _i in 0..200 {
        eris_levels = evolve_eris(&mut eris_levels);
    }

    println!("{:?}", eris_levels);

    let mut bugs : u32 = 0;
    for e in eris_levels.iter() {
        bugs += count_bugs(*e.1);
    }

    println!("Solution bugs: {}", bugs);
}
