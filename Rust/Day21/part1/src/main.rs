use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_map(map : &Vec<Vec<i8>>) {
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let c = if map[r][c] == 1 { '#' } 
                    else { '.' };
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Clone)]
struct Intcode {
    map : Vec<Vec<i8>>,
    comm : Vec<String>,
    r : usize,
    c : usize
}

// jmp table
// a b c d
// 0 0 0 0   0
// 0 0 0 1   1
// 0 0 1 0   2
// 0 0 1 1   3
// 0 1 0 0   4
// 0 1 0 1   5
// 0 1 1 0   6
// 0 1 1 1   7
// 1 0 0 0   8
// 1 0 0 1   9
// 1 0 1 0  10 
// 1 0 1 1  11
// 1 1 0 0  12
// 1 1 0 1  13
// 1 1 1 0  14
// 1 1 1 1  15
// 
// formula
// (~A * D) + (A * D * (~B + ~C))
// reduction
// (~A + ~B + ~C) * D
// input
// NOT A J
// NOT B T
// OR T J
// NOT C T
// OR T J
// AND D J
// 0 0 0 0 -> loss
// 0 0 0 1 -> jmp
// 0 0 1 0 -> loss
// 0 0 1 1 -> jmp
// 0 1 0 0 -> loss
// 0 1 0 1 -> jmp
// 0 1 1 0 -> loss
// 0 1 1 1 -> jmp
// 1 0 0 0 -> no jmp
// 1 0 0 1 -> jmp
// 1 0 1 0 -> no jmp
// 1 0 1 1 -> jmp
// 1 1 0 0 -> no jmp
// 1 1 0 1 -> jmp
// 1 1 1 0 -> no jmp
// 1 1 1 1 -> no jmp

impl Intcode {
    fn new() -> Intcode {
        Intcode {
            map : vec![Vec::new()],
            comm : vec!["NOT A J".to_string(), 
                        "NOT B T".to_string(), 
                        "OR T J".to_string(), 
                        "NOT C T".to_string(), 
                        "OR T J".to_string(),
                        "AND D J".to_string(),
                        "WALK".to_string()],
            r : 0,
            c : 0
        }
    }

    fn read_input(&mut self, _count : i64) -> i64 {
        let mut v : i64 = 0;
        if self.r < self.comm.len() {
            if self.c == self.comm[self.r].as_bytes().len() {
                self.c = 0;
                self.r += 1;
                v = 10;
            } else {
                v = self.comm[self.r].as_bytes()[self.c] as i64;
                self.c += 1;
            }
        }

        v
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        if 0 <= value && value < 256 {
            let v = [value as u8];
            let v_str = std::str::from_utf8(&v).unwrap();
            print!("{}", v_str);
        } else {
            println!("output: {}", value);
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

    commands.resize(3000, 0);
    let mut intcode = Intcode::new();
    emulate(&mut commands.clone(), &mut intcode);
    
    // println!("Origin: ({},{})", origin_x, origin_y);
    // print_map(&intcode.map);
}
