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

    const MAX_STEPS : usize = 1000;
    for _steps in 1..=MAX_STEPS {
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

        // println!("Step {}", _steps);
        let mut moons_total = 0;
        for i in 0..moons.len() {
            moons[i].0 += velocity[i].0;
            moons[i].1 += velocity[i].1;
            moons[i].2 += velocity[i].2;
            // println!("pos<{:?}>, vel<{:?}>", moons[i], velocity[i]);
            let pot = moons[i].0.abs() + moons[i].1.abs() + moons[i].2.abs();
            let kin = velocity[i].0.abs() + velocity[i].1.abs() + velocity[i].2.abs();
            let total = pot * kin;
            // println!("pot: {}, kin: {} -> total: {}", pot, kin, total);
             moons_total += total;
        }
        if _steps == MAX_STEPS {
            println!("total energy: {}", moons_total);
        }
        // println!();
    }
}
