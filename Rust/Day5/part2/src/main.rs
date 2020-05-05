use std::io::{self,Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(count : i32) -> i32 {
    loop {
        print!("{} - Input: ", count);
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => return i,
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
    }
}

fn get_parameter(commands: &Vec<i32>, ip : usize, mode : i32) -> i32 {
    match mode {
        0 => return commands[commands[ip] as usize],
        1 => return commands[ip],
        _ => println!("Invalid mode: {}, ip: {}, command: {}, commands: {:?}", mode, ip, commands[ip], commands),
    }
    return 0;
}

fn emulate(commands: &mut Vec<i32>) -> i32 {
    let mut ip = 0;
    let mut count = 0;
    loop {
        if ip >= commands.len() {
            println!("Outside memory");
            break;
        }
        count += 1;
        let full_opcode =commands[ip];
        let opcode = full_opcode % 100;
        let mode1 = (full_opcode /   100) % 10;
        let mode2 = (full_opcode /  1000) % 10;
        let mode3 = (full_opcode / 10000) % 10;
        match opcode {
            1 => {
                if ip + 4 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let param1 = get_parameter(commands, ip + 1, mode1);
                let param2 = get_parameter(commands, ip + 2, mode2);
                let new_value = param1 + param2;
                assert_eq!(mode3, 0);

                let dst = commands[ip + 3] as usize;
                commands[dst] = new_value;
                ip += 4;
            },
            2 => {
                if ip + 4 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let param1 = get_parameter(commands, ip + 1, mode1);
                let param2 = get_parameter(commands, ip + 2, mode2);
                let new_value = param1 * param2;
                assert_eq!(mode3, 0);

                let dst = commands[ip + 3] as usize;
                commands[dst] = new_value;
                ip += 4;
            },
            3 => {
                if ip + 2 > commands.len() {
                    println!("Outside memory");
                    break;
                }

                let value = read_input(count);
                let input_idx = commands[ip + 1] as usize;
                commands[input_idx] = value;
                ip += 2;
            },
            4 => {
                if ip + 2 > commands.len() {
                    println!("Outside memory");
                    break;
                }

                let value = get_parameter(commands, ip + 1, mode1);
                println!("{} - Output: {}", count, value);
                ip += 2;
            },
            5 => {
                if ip + 3 > commands.len() {
                    println!("Outside memory");
                    break;
                }

                let param1 = get_parameter(commands, ip + 1, mode1);
                if param1 != 0 {
                    let value = get_parameter(commands, ip + 2, mode2);
                    if value < 0 {
                        println!("Invalid jump address {}", value);
                    }
                    ip = value as usize;
                } else {
                    ip += 3;
                }
            },
            6 => {
                if ip + 3 > commands.len() {
                    println!("Outside memory");
                    break;
                }

                let param1 = get_parameter(commands, ip + 1, mode1);
                if param1 == 0 {
                    let value = get_parameter(commands, ip + 2, mode2);
                    if value < 0 {
                        println!("Invalid jump address {}", value);
                    }
                    ip = value as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                if ip + 4 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let param1 = get_parameter(commands, ip + 1, mode1);
                let param2 = get_parameter(commands, ip + 2, mode2);
                let new_value = if param1 < param2 { 1 } else { 0 };
                assert_eq!(mode3, 0);

                let dst = commands[ip + 3] as usize;
                commands[dst] = new_value;
                ip += 4;
            },
            8 => {
                if ip + 4 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let param1 = get_parameter(commands, ip + 1, mode1);
                let param2 = get_parameter(commands, ip + 2, mode2);
                let new_value = if param1 == param2 { 1 } else { 0 };
                assert_eq!(mode3, 0);

                let dst = commands[ip + 3] as usize;
                commands[dst] = new_value;
                ip += 4;
            },

            99 => {
                println!("{} - Halting", count);
                return commands[0];
            },
            _ => println!("Invalid command: {}, ip: {}, commands: {:?}", commands[ip], ip, commands),
        }
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

    emulate(&mut commands);
}
