extern crate num;

use num::Integer;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn gravity(m1 : &(i64, i64, i64), m2 : &(i64, i64, i64)) -> (i64, i64, i64) {
    let mut gx = m2.0 - m1.0;
    if gx != 0 {
        gx /= (m2.0 - m1.0).abs();
    }
    let mut gy = m2.1 - m1.1;
    if gy != 0 {
        gy /= (m2.1 - m1.1).abs();
    }
    let mut gz = m2.2 - m1.2;
    if gz != 0 {
        gz /= (m2.2 - m1.2).abs();
    }
    (gx, gy, gz)
}

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut moons = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.trim().len() == 0 {
            break;
        }
        // Show the line and its number.
        let mut commands_str = line.split(",");
        let x : i64 = commands_str.next().unwrap()[3..].parse().unwrap();
        let y : i64 = commands_str.next().unwrap()[3..].parse().unwrap();
        let z_str = commands_str.next().unwrap();
        let z : i64 = z_str[3..z_str.len() - 1].parse().unwrap();
        // println!("({}, {}, {})", x, y, z);

        moons.push((x, y, z));
    }

    let mut velocity : Vec<(i64, i64, i64)> = vec![(0,0,0); moons.len()];

    let mut seen = HashSet::new();

    seen.insert(('x', moons[0].0, moons[1].0, moons[2].0, moons[3].0, velocity[0].0, velocity[1].0, velocity[2].0, velocity[3].0));
    seen.insert(('y', moons[0].1, moons[1].1, moons[2].1, moons[3].1, velocity[0].1, velocity[1].1, velocity[2].1, velocity[3].1));
    seen.insert(('z', moons[0].2, moons[1].2, moons[2].2, moons[3].2, velocity[0].2, velocity[1].2, velocity[2].2, velocity[3].2));

    let mut count_x : i64 = 0;
    let mut count_y : i64 = 0;
    let mut count_z : i64 = 0;
    
    let original_moons_positions = moons.clone();
    let original_moons_velocity = velocity.clone();
    let mut counter = 0;
    loop {
        counter += 1;
        for i in 0..moons.len() {
            let mut gravity_i = (0i64,0i64,0i64);
            for j in 0..moons.len() {
                if i != j {
                    let g = gravity(&moons[i], &moons[j]);
                    gravity_i.0 += g.0;
                    gravity_i.1 += g.1;
                    gravity_i.2 += g.2;
                    // println!("{}, {} -> {:?}", i, j, g);
                }
            }
            velocity[i].0 += gravity_i.0;
            velocity[i].1 += gravity_i.1;
            velocity[i].2 += gravity_i.2;
        }

        // println!("Step {}", counter);
        for i in 0..moons.len() {
            moons[i].0 += velocity[i].0;
            moons[i].1 += velocity[i].1;
            moons[i].2 += velocity[i].2;
            // println!("pos<{:?}>, vel<{:?}>", moons[i], velocity[i]);
        }
        // println!();

        if count_x == 0 && !seen.insert(('x', moons[0].0, moons[1].0, moons[2].0, moons[3].0, velocity[0].0, velocity[1].0, velocity[2].0, velocity[3].0)) {
            println!("step: {}, data: {:?}", counter, ('x', moons[0].0, moons[1].0, moons[2].0, moons[3].0, velocity[0].0, velocity[1].0, velocity[2].0, velocity[3].0));
            count_x = counter;
            if count_y != 0 && count_z != 0 {
                break;
            }
        }
        if count_y == 0 && !seen.insert(('y', moons[0].1, moons[1].1, moons[2].1, moons[3].1, velocity[0].1, velocity[1].1, velocity[2].1, velocity[3].1)) {
            println!("step: {}, data: {:?}", counter, ('y', moons[0].1, moons[1].1, moons[2].1, moons[3].1, velocity[0].1, velocity[1].1, velocity[2].1, velocity[3].1));
            count_y = counter;
            if count_x != 0 && count_z != 0 {
                break;
            }
        }
        if count_z == 0 && !seen.insert(('z', moons[0].2, moons[1].2, moons[2].2, moons[3].2, velocity[0].2, velocity[1].2, velocity[2].2, velocity[3].2)) {
            println!("step: {}, data: {:?}", counter, ('z', moons[0].2, moons[1].2, moons[2].2, moons[3].2, velocity[0].2, velocity[1].2, velocity[2].2, velocity[3].2));
            count_z = counter;
            if count_x != 0 && count_y != 0 {
                break;
            }
        }
        
        if original_moons_positions == moons && original_moons_velocity == velocity {
            println!("{}", counter);
            break;
        }

        if counter % 100_000_000 == 0 {
            println!("{}", counter);
        }
    }

    let solution = count_x.lcm(&count_y).lcm(&count_z);
    println!("Solution: {}", solution);
}
