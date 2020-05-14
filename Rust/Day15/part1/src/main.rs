use std::fs::File;
use std::collections::HashMap;
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

fn print_map(map : &[u8], height : usize, width: usize, origin : &(usize, usize), actual : &(usize, usize, usize)) {
    println!("origin: {:?}, actual: {:?}", origin, actual);
    for r in 0..height {
        for c in 0..width {
            let c = if r == origin.0 && c == origin.1 { 'X' } 
                    else if r == actual.0 && c == actual.1 { 'o' } 
                    else if map[r*width + c] == 0 { ' ' }
                    else if map[r*width + c] == 1 { '#' }
                    else if map[r*width + c] == 9 { '?' }
                    else { '0' };
            print!("{}", c);
        }
        println!();
    }
}

const D : [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

//north (1), south (2), west (3), and east (4)
//0: empty, 1: wall, 2: repair station, 9: unexplore

const MAP_ROW : usize = 50;
const MAP_COL : usize = 60;

struct Intcode {
    map : Vec<u8>,
    robot_move : Vec<(usize, usize, usize)>,
    best_sol : usize,
    counter : usize,
    revert_move : bool
}

impl Intcode {
    fn new() -> Intcode {
        Intcode {
            map : vec![9u8; MAP_ROW * MAP_COL],
            robot_move : vec![(MAP_ROW / 2, MAP_COL / 2, 0)],
            best_sol : usize::max_value(),
            counter : 0,
            revert_move : false
        }
    }

    fn revert_move(&mut self) -> i64 {
        self.robot_move.remove(self.robot_move.len() - 1);
        if self.robot_move.len() == 0 {
            println!("Solution: {}", self.best_sol);
            print_map(&self.map, MAP_ROW, MAP_COL, &(MAP_ROW / 2, MAP_COL / 2), &(0, 0, 0));
            std::process::exit(0);
        }
        let prev_move = self.robot_move.last().unwrap().2;
        let rev_move = if prev_move % 2 == 0 { prev_move - 1 } else { prev_move + 1 };
        self.revert_move = true;
        // println!("{:?} <- {}", self.robot_move.last().unwrap(), rev_move);        
        return rev_move as i64;
    }

    fn read_input(&mut self, _count : i64) -> i64 {
        if self.robot_move.last().unwrap().2 >= 4 {
            return self.revert_move();
        }
        self.robot_move.last_mut().unwrap().2 += 1;
        let mut actual_move = self.robot_move.last().unwrap().2;
        if self.robot_move.len() > 1 {
            let prev_move = self.robot_move[self.robot_move.len() - 2].2;
            let rev_move = if prev_move % 2 == 0 { prev_move - 1 } else { prev_move + 1 };
            if actual_move == rev_move {
                if actual_move >= 4 {
                    return self.revert_move();
                } else {
                    self.robot_move.last_mut().unwrap().2 += 1;
                    actual_move += 1;
                }
            }
        }
        return actual_move as i64;
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        let last_info = self.robot_move.last().unwrap();
        let last_pos = (last_info.0, last_info.1);
        let last_move = last_info.2 - 1;
        let new_pos = ((last_pos.0 as i64 + D[last_move].0) as usize, (last_pos.1 as i64 + D[last_move].1) as usize);

        // print!("last pos: {:?}, move: {}, ", last_pos, last_move + 1);

        match value {
            0 => {
                // println!("BLOCK: {:?}", new_pos);
                self.map[new_pos.0 * MAP_COL + new_pos.1] = 1u8;
            },
            1 => {
                if self.revert_move {
                    self.revert_move = false;
                } else {
                    self.map[new_pos.0 * MAP_COL + new_pos.1] = 0u8;
                    self.robot_move.push((new_pos.0, new_pos.1, 0));
                    // println!("NEW: {:?}", self.robot_move.last().unwrap());
                }
            },
            2 => {
                self.map[new_pos.0 * MAP_COL + new_pos.1] = 2u8;
                self.robot_move.push((new_pos.0, new_pos.1, 4));
                if self.robot_move.len() - 1 < self.best_sol {
                    self.best_sol = self.robot_move.len() - 1;
                    
                }
                println!("Found: {}", self.best_sol);
                print_map(&self.map, MAP_ROW, MAP_COL, &(MAP_ROW / 2, MAP_COL / 2), &(0, 0, 0));
                println!("Moves: {:?}", self.robot_move);
            },
            _ => unreachable!()
        }
        // self.counter += 1;
        // if self.counter % 100 == 0 {
        //     print_map(&self.map, MAP_ROW, MAP_COL, &(MAP_ROW / 2, MAP_COL / 2), self.robot_move.last().unwrap());
        // }
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

    commands.resize(2000, 0);
    let mut intcode = Intcode::new();
    emulate(&mut commands, &mut intcode);

    save_tgc(&intcode.map, MAP_ROW, MAP_COL);
    // println!("Panel painted: {:?}", intcode.paint_panels);
}
