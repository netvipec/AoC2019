use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn get_parameter(commands: &Vec<i32>, ip : usize, mode : i32) -> i32 {
    match mode {
        0 => return commands[commands[ip] as usize],
        1 => return commands[ip],
        _ => println!("Invalid mode: {}, ip: {}, command: {}, commands: {:?}", mode, ip, commands[ip], commands),
    }
    return 0;
}

fn emulate(commands: &mut Vec<i32>, phase_setting : i32, input_signal : i32) -> i32 {
    let mut ip = 0;
    let mut count = 0;
    let mut input_commands = 0;
    let mut output_value = 0;
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

                let value = if input_commands == 0 { phase_setting } else { input_signal };
                input_commands += 1;
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
                output_value = value;
                // println!("{} - Output: {}", count, value);
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
                // println!("{} - Halting", count);
                return output_value;
            },
            _ => println!("Invalid command: {}, ip: {}, commands: {:?}", commands[ip], ip, commands),
        }
    }
    return 0;
}

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

    let mut max_output_amplifier = 0;
    let phase_setting : Vec<i32> = vec![0, 1, 2, 3, 4];
    let mut best_perm : Vec<&i32> = Vec::new();
    for perm in phase_setting.iter().permutations(phase_setting.len()) {
        let mut input_signal = 0;
        for ps in &perm {
            let mut commands_copy = commands.clone();

            input_signal = emulate(&mut commands_copy, **ps, input_signal);
        }
        // println!("Output for perm {:?} -> {}", perm, input_signal);

        if input_signal > max_output_amplifier {
            best_perm = perm;
            max_output_amplifier = input_signal;
        }
    }
    println!("Solution: {}", max_output_amplifier);
    println!("Best Phase Setting permutation: {:?}", best_perm);
}
