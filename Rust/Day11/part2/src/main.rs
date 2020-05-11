use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

// typedef struct {
//     char  idlength;
//     char  colourmaptype;
//     char  datatypecode;              // 2
//     short int colourmaporigin;
//     short int colourmaplength;
//     char  colourmapdepth;
//     short int x_origin;
//     short int y_origin;
//     short width;
//     short height;
//     char  bitsperpixel;
//     char  imagedescriptor;
//  } HEADER;

fn save_tgc(image : &[u8], height : usize, width : usize) -> std::io::Result<()> {
    let mut file = File::create("message.tga")?;

    let tga_header = [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, (width & 0x00FF) as u8, ((width & 0xFF00) / 256) as u8, (height & 0x00FF) as u8, ((height & 0xFF00) / 256) as u8, 24, 0];
    file.write_all(&tga_header)?;
    for r in (0..height).rev() {
        for c in 0..width {
            let color : u8 = if image[r * width + c] == 0 { 0 } else { 0xff };
            file.write_all(&[color, color, color])?;
        }
    }
    Ok(())
}

const D : [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

const MAP_ROW : usize = 20;
const MAP_COL : usize = 80;

struct Intcode {
    map : Vec<u8>,
    robot_pos : (usize, usize),
    robot_dir : usize, // U R D L
    output_count : usize
}

impl Intcode {
    fn new() -> Intcode {
        let mut res = Intcode {
            map : vec![0u8; MAP_ROW * MAP_COL],
            robot_pos : (MAP_ROW / 2, MAP_COL / 2),
            robot_dir : 0,
            output_count : 0
        };
        res.map[res.robot_pos.0 * MAP_COL + res.robot_pos.1] = 1;
        return res;
    }

    fn read_input(&self, _count : i64) -> i64 {
        return self.map[self.robot_pos.0 * MAP_COL + self.robot_pos.1] as i64;
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        if self.output_count % 2 == 0 {
            // println!("output {}, value: {}", _count, value);
            self.map[self.robot_pos.0 * MAP_COL + self.robot_pos.1] = value as u8;
        } else {
            // println!("output {}, value: {}", _count, value);
            match value {
                0 => { self.robot_dir += 3; self.robot_dir %= 4; },
                1 => { self.robot_dir += 1; self.robot_dir %= 4; },
                _ => unreachable!(),
            };

            self.robot_pos.0 = (D[self.robot_dir].0 + self.robot_pos.0 as i32) as usize;
            self.robot_pos.1 = (D[self.robot_dir].1 + self.robot_pos.1 as i32) as usize;

            println!("robot pos {:?}, robot dir: {}", self.robot_pos, self.robot_dir);
        }
        self.output_count += 1;
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
            let comm: i64 = comm_str.parse().unwrap();
            commands.push(comm);
        }
        break;
    }

    commands.resize(2000, 0);

    let mut intcode = Intcode::new();

    emulate(&mut commands, &mut intcode);

    for r in 0..MAP_ROW {
        for c in 0..MAP_COL {
            print!("{}", if intcode.map[r*MAP_COL + c] == 0 { ' ' } else { '0' });
        }
        println!();
    }

    save_tgc(&intcode.map, MAP_ROW, MAP_COL);
    // println!("Panel painted: {:?}", intcode.paint_panels);
}
