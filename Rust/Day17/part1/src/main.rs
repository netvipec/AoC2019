use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_map(map : &Vec<Vec<i8>>) {
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let c = if map[r][c] == 35 { '#' } else if map[r][c] == 46 { '.' } else { '0' };
            print!("{}", c);
        }
        println!();
    }
}

struct Intcode {
    map : Vec<Vec<i8>>,
    y : usize
}

impl Intcode {
    fn new() -> Intcode {
        Intcode {
            map : vec![Vec::new()],
            y : 0
        }
    }

    fn read_input(&mut self, _count : i64) -> i64 {
        return 0;
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        if value == 10 {
            self.map.push(Vec::new());
            self.y = 0;
        } else {
            self.map.last_mut().unwrap().push(value as i8);
            self.y += 1;
        }
    }
}

fn get_parameter(commands: &Vec<i64>, ip : usize, mode : i64, relative_base : i64) -> i64 {
    match mode {
        0 => return commands[commands[ip] as usize],
        1 => return commands[ip],
        2 => return commands[(relative_base + commands[ip]) as usize],
        _ => println!("Invalid mode: {}, ip: {}, command: {}, commands: {:?}", mode, ip, commands[ip], commands),
    }
    return 0;
}

fn set_parameter(commands: &mut Vec<i64>, ip : usize, mode : i64, relative_base : i64, value : i64) {
    match mode {
        0 => {
            let input_idx = commands[ip] as usize;
            commands[input_idx] = value;
        },
        1 => println!("Invalid mode: {}, ip: {}, command: {}, commands: {:?}", mode, ip, commands[ip], commands),
        2 => {
            let input_idx = (relative_base + commands[ip]) as usize;
            commands[input_idx] = value;
        },
        _ => println!("Invalid mode: {}, ip: {}, command: {}, commands: {:?}", mode, ip, commands[ip], commands),
    };
}

fn emulate(commands: &mut Vec<i64>, intcode : &mut Intcode) -> i64 {
    let mut ip = 0;
    let mut count = 0;
    let mut relative_base = 0;
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
                let param1 = get_parameter(commands, ip + 1, mode1, relative_base);
                let param2 = get_parameter(commands, ip + 2, mode2, relative_base);
                let new_value = param1 + param2;
                set_parameter(commands, ip + 3, mode3, relative_base, new_value);
                ip += 4;
            },
            2 => {
                if ip + 4 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let param1 = get_parameter(commands, ip + 1, mode1, relative_base);
                let param2 = get_parameter(commands, ip + 2, mode2, relative_base);
                let new_value = param1 * param2;
                set_parameter(commands, ip + 3, mode3, relative_base, new_value);
                ip += 4;
            },
            3 => {
                if ip + 2 > commands.len() {
                    println!("Outside memory");
                    break;
                }

                let value = intcode.read_input(count);
                set_parameter(commands, ip + 1, mode1, relative_base, value);
                ip += 2;
            },
            4 => {
                if ip + 2 > commands.len() {
                    println!("Outside memory");
                    break;
                }

                let value = get_parameter(commands, ip + 1, mode1, relative_base);
                intcode.write_output(count, value);
                ip += 2;
            },
            5 => {
                if ip + 3 > commands.len() {
                    println!("Outside memory");
                    break;
                }

                let param1 = get_parameter(commands, ip + 1, mode1, relative_base);
                if param1 != 0 {
                    let value = get_parameter(commands, ip + 2, mode2, relative_base);
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

                let param1 = get_parameter(commands, ip + 1, mode1, relative_base);
                if param1 == 0 {
                    let value = get_parameter(commands, ip + 2, mode2, relative_base);
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
                let param1 = get_parameter(commands, ip + 1, mode1, relative_base);
                let param2 = get_parameter(commands, ip + 2, mode2, relative_base);
                let new_value = if param1 < param2 { 1 } else { 0 };
                set_parameter(commands, ip + 3, mode3, relative_base, new_value);
                ip += 4;
            },
            8 => {
                if ip + 4 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let param1 = get_parameter(commands, ip + 1, mode1, relative_base);
                let param2 = get_parameter(commands, ip + 2, mode2, relative_base);
                let new_value = if param1 == param2 { 1 } else { 0 };                
                set_parameter(commands, ip + 3, mode3, relative_base, new_value);
                ip += 4;
            },
            9 => {
                if ip + 2 > commands.len() {
                    println!("Outside memory");
                    break;
                }
                let param1 = get_parameter(commands, ip + 1, mode1, relative_base);
                relative_base += param1;
                ip += 2;
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

const D : [(i8, i8);4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_aligment(map : &mut Vec<Vec<i8>>) -> i64 {
    let mut sum : i64 = 0;
    for r in 1..map.len() - 1 {
        for c in 1..map[r].len() - 1 {
            if map[r][c] == 35 {
                let mut counter = 0;
                for d in D.iter() {
                    let new_r = ((r as i8) + d.0) as usize;
                    let new_c = ((c as i8) + d.1) as usize;
                    // println!("nr: {}, nc: {}, size: {},{}", new_r, new_c, map.len(), map[r].len());
                    if map[new_r][new_c] == 35 {
                        counter += 1;
                    }
                }
                if counter == 4 {
                    map[r][c] = '0' as i8;
                    sum += (r as i64) * (c as i64);
                }
            }
        }
    }
    sum
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
            let comm: i64 = comm_str.parse().unwrap();
            commands.push(comm);
        }
        break;
    }

    commands.resize(4000, 0);
    let mut intcode = Intcode::new();
    emulate(&mut commands, &mut intcode);
    intcode.map.remove(intcode.map.len() - 1);
    intcode.map.remove(intcode.map.len() - 1);

    println!("Printing map: ");
    print_map(&intcode.map);

    let mut map = intcode.map.clone();
    let sum = get_aligment(&mut map);

    println!("Printing aligmnet map: ");
    print_map(&map);

    println!("Solution part1: {}", sum);
}
