use std::fs::File;
use std::io::{BufRead, BufReader};

fn emulate(commands: &mut Vec<i32>) -> i32 {
    let mut ip = 0;
    loop {
        if ip >= commands.len() {
            println!("Outside memory");
            break;
        }
        match commands[ip] {
            1 => {
                if ip + 4 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let new_value = commands[commands[ip + 1] as usize] + commands[commands[ip + 2] as usize];
                let dst = commands[ip + 3] as usize;
                commands[dst] = new_value
            },
            2 => {
                if ip + 4 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let new_value = commands[commands[ip + 1] as usize] * commands[commands[ip + 2] as usize];
                let dst = commands[ip + 3] as usize;
                commands[dst] = new_value;
            },
            99 => {
                // println!("Solution: {}", commands[0]); 
                // println!("Commands: {:?}", commands);
                return commands[0];
            },
            _ => println!("Invalid command: {}, ip: {}, commands: {:?}", commands[ip], ip, commands),
        }

        ip += 4;
    }
    return 0;
}

fn main() {
    let filename = "../part1/src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut commands = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        let commands_str = line.split(",");
        for comm_str in commands_str {
            let comm: i32 = comm_str.parse().unwrap();
            commands.push(comm);
        }
        break;
    }

    for noun in 0..99 + 1 {
        for verb in 0..99 + 1 {
            let mut new_commands = commands.to_vec();

            new_commands[1] = noun;
            new_commands[2] = verb;

            if emulate(&mut new_commands) == 19690720 {
                println!("Noun: {}, Verb: {}", noun, verb);
                println!("Solution: {}", 100 * noun + verb);
                return;
            }
        }
    }
}
