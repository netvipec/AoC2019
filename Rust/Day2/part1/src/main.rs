use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input";
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

    commands[1] = 12;
    commands[2] = 2;

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
                println!("Solution: {}", commands[0]); 
                println!("Commands: {:?}", commands);
                return;
            },
            _ => println!("Invalid command: {}, ip: {}, commands: {:?}", commands[ip], ip, commands),
        }

        ip += 4;
    }
}
