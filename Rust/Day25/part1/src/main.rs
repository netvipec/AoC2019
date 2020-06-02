use std::io::{self,Write};
use std::fs::File;
use std::collections::{HashSet,HashMap};
use std::io::{BufRead, BufReader};
use std::process;

const MAX_SIZE_CMD : usize = 100;

const D : [(i32, i32);4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone)]
struct Intcode {
    last_cmd : String,
    last_cmd_idx : usize,

    writed_output : String,
    path : Vec<(String, String, (i32,i32))>,
    visit_points : HashSet<(i32, i32)>,
    pos : (i32, i32),
    str_to_d : HashMap<String, usize>,
    rooms : HashMap<(i32, i32), String>,
    last_tested : String
}

impl Intcode {
    fn new() -> Intcode {
        let mut ic = Intcode {
            last_cmd : String::new(),
            last_cmd_idx : MAX_SIZE_CMD,
            writed_output : String::new(),
            path : Vec::new(),
            visit_points : HashSet::new(),
            pos : (0i32, 0i32),
            str_to_d : HashMap::new(),
            rooms : HashMap::new(),
            last_tested : String::new()
        };

        ic.str_to_d.insert("north".to_string(), 0);
        ic.str_to_d.insert("south".to_string(), 1);
        ic.str_to_d.insert("west".to_string(), 2);
        ic.str_to_d.insert("east".to_string(), 3);

        ic.path.push(("".to_string(), "".to_string(), (0,0)));

        return ic;
    }

    fn read_input(&mut self, _count : i64) -> i64 {
        if self.last_cmd.len() > 0 && self.last_cmd_idx <= self.last_cmd.len() {
            if let Some(c) = self.last_cmd.as_bytes().get(self.last_cmd_idx) {
                self.last_cmd_idx += 1;
                print!("{}", *c as char);
                return *c as i64;
            } else {
                self.last_cmd.clear();
                self.last_cmd_idx = MAX_SIZE_CMD;
                println!();
                return 10;
            }
            
        } else {
            // manual input
            print!("{} - Input: ", _count);
            io::stdout().flush().ok().expect("Could not flush stdout");

            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            // let mut valid_d = Vec::new();
            // let mut read_doors = false;
            // for line in self.writed_output.lines() {
            //     if line == "Doors here lead:" {
            //         read_doors = true;
            //     } else if read_doors {
            //         if line == "- north" {
            //             valid_d.push(("north".to_string(), "south".to_string()));
            //         } else if line == "- east" {
            //             valid_d.push(("east".to_string(), "west".to_string()));
            //         } else if line == "- south" {
            //             valid_d.push(("south".to_string(), "north".to_string()));
            //         } else if line == "- west" {
            //             valid_d.push(("west".to_string(), "east".to_string()));
            //         } else {
            //             read_doors = false;
            //         }
            //     }
            // }

            // if !self.visit_points.contains(&self.pos) {
            //     self.visit_points.insert(self.pos);
            //     self.rooms.insert(self.pos, self.writed_output.clone());
            // }

            // let mut move_direction = String::new();
            // let mut new_pos = (0i32, 0i32);
            // let last_d = match self.path.last() {
            //     Some(d) => d.1.clone(),
            //     None => "".to_string()
            // };
            // let rev_pos = match valid_d.iter().position(|a| str::eq(&a.0, &self.last_tested)) {
            //     Some(p) => p,
            //     None => 5
            // };
            // let for_pos = match valid_d.iter().position(|a| str::eq(&a.0, &last_d)) {
            //     Some(p) => p,
            //     None => 5
            // };
            // for vdi in 0..valid_d.len() {
            //     if vdi == rev_pos || vdi == for_pos {
            //         continue;
            //     }
            //     let vd = &valid_d[vdi];
            //     let di = *self.str_to_d.get(&vd.0).unwrap();

            //     new_pos.0 = self.pos.0 + D[di].0;
            //     new_pos.1 = self.pos.1 + D[di].1;

            //     if !self.visit_points.contains(&new_pos) {
            //         move_direction = vd.0.clone();
            //         self.path.push((vd.0.clone(), vd.1.clone(), self.pos));
            //         self.last_tested.clear();
            //         self.pos = new_pos;
            //         break;
            //     }
            // }

            // if move_direction == "" {
            //     let last_idx = self.path.len() - 1;
            //     move_direction = (*self.path.get(last_idx).unwrap()).1.clone();
            //     self.last_tested = (*self.path.get(last_idx).unwrap()).0.clone();
            //     self.pos = (*self.path.get(last_idx).unwrap()).2;
            //     self.path.pop();
            //     if self.path.len() == 0 {
            //         println!("rooms: =============================================");
            //         for r in self.rooms.iter() {
            //             println!("{:?} {}", r.0, r.1);
            //         }
                    
            //         process::exit(0);
            //     }
            // }

            // self.last_cmd = move_direction.clone();
            // println!("Input to use: {}", move_direction);
            self.last_cmd = input_text.trim().to_string();

            if self.last_cmd == "north" {
                self.pos.0 += D[0].0;
                self.pos.1 += D[0].1;
            } else if self.last_cmd == "south" {
                self.pos.0 += D[1].0;
                self.pos.1 += D[1].1;
            } else if self.last_cmd == "west" {
                self.pos.0 += D[2].0;
                self.pos.1 += D[2].1;
            } else if self.last_cmd == "east" {
                self.pos.0 += D[3].0;
                self.pos.1 += D[3].1;
            }

            println!("pos: {:?}", self.pos);

            self.last_cmd_idx = 1;
            // self.writed_output.clear();

            let c = *self.last_cmd.as_bytes().get(0).unwrap();
            print!("Input send: {}", c as char);
            return c as i64;
        }
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        if 0 <= value && value < 256 {
            let c = (value as u8) as char;
            self.writed_output.push(c);
            print!("{}", c);
        } else {
            println!("{}", value);
        }
    }
}

#[derive(Clone)]
struct IntcodeEmulator {
    ip : usize,
    count: i64,
    relative_base : i64,
    commands : Vec<i64>,
    intcode : Intcode
}

impl IntcodeEmulator {
    fn new(commands: Vec<i64>, intcode : Intcode) -> IntcodeEmulator {
        IntcodeEmulator {
            ip : 0,
            count : 0,
            relative_base : 0,
            commands : commands,
            intcode : intcode
        }
    }

    fn get_parameter(&self, ip : usize, mode : i64) -> i64 {
        match mode {
            0 => return self.commands[self.commands[ip] as usize],
            1 => return self.commands[ip],
            2 => return self.commands[(self.relative_base + self.commands[ip]) as usize],
            _ => println!("Invalid mode: {}, ip: {}, command: {}, commands: {:?}", mode, ip, self.commands[ip], self.commands),
        }
        return 0;
    }
    
    fn set_parameter(&mut self, ip : usize, mode : i64, value : i64) {
        match mode {
            0 => {
                let input_idx = self.commands[ip] as usize;
                self.commands[input_idx] = value;
            },
            1 => println!("Invalid mode: {}, ip: {}, command: {}, commands: {:?}", mode, ip, self.commands[ip], self.commands),
            2 => {
                let input_idx = (self.relative_base + self.commands[ip]) as usize;
                self.commands[input_idx] = value;
            },
            _ => println!("Invalid mode: {}, ip: {}, command: {}, commands: {:?}", mode, ip, self.commands[ip], self.commands),
        };
    }

    fn emulate(&mut self) -> Option<i64> {
        let mut r = None;
        if self.ip >= self.commands.len() {
            println!("Outside memory");
            return r;
        }
        self.count += 1;
        let full_opcode = self.commands[self.ip];
        let opcode = full_opcode % 100;
        let mode1 = (full_opcode /   100) % 10;
        let mode2 = (full_opcode /  1000) % 10;
        let mode3 = (full_opcode / 10000) % 10;
        match opcode {
            1 => {
                if self.ip + 4 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }
                let param1 = self.get_parameter(self.ip + 1, mode1);
                let param2 = self.get_parameter(self.ip + 2, mode2);
                let new_value = param1 + param2;
                self.set_parameter(self.ip + 3, mode3, new_value);
                self.ip += 4;
            },
            2 => {
                if self.ip + 4 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }
                let param1 = self.get_parameter(self.ip + 1, mode1);
                let param2 = self.get_parameter(self.ip + 2, mode2);
                let new_value = param1 * param2;
                self.set_parameter(self.ip + 3, mode3, new_value);
                self.ip += 4;
            },
            3 => {
                if self.ip + 2 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }

                let value = self.intcode.read_input(self.count);
                self.set_parameter(self.ip + 1, mode1, value);
                self.ip += 2;
            },
            4 => {
                if self.ip + 2 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }

                let value = self.get_parameter(self.ip + 1, mode1);
                self.intcode.write_output(self.count, value);
                r = Some(value);
                self.ip += 2;
            },
            5 => {
                if self.ip + 3 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }

                let param1 = self.get_parameter(self.ip + 1, mode1);
                if param1 != 0 {
                    let value = self.get_parameter(self.ip + 2, mode2);
                    if value < 0 {
                        println!("Invalid jump address {}", value);
                    }
                    self.ip = value as usize;
                } else {
                    self.ip += 3;
                }
            },
            6 => {
                if self.ip + 3 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }

                let param1 = self.get_parameter(self.ip + 1, mode1);
                if param1 == 0 {
                    let value = self.get_parameter(self.ip + 2, mode2);
                    if value < 0 {
                        println!("Invalid jump address {}", value);
                    }
                    self.ip = value as usize;
                } else {
                    self.ip += 3;
                }
            },
            7 => {
                if self.ip + 4 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }
                let param1 = self.get_parameter(self.ip + 1, mode1);
                let param2 = self.get_parameter(self.ip + 2, mode2);
                let new_value = if param1 < param2 { 1 } else { 0 };
                self.set_parameter(self.ip + 3, mode3, new_value);
                self.ip += 4;
            },
            8 => {
                if self.ip + 4 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }
                let param1 = self.get_parameter(self.ip + 1, mode1);
                let param2 = self.get_parameter(self.ip + 2, mode2);
                let new_value = if param1 == param2 { 1 } else { 0 };                
                self.set_parameter(self.ip + 3, mode3, new_value);
                self.ip += 4;
            },
            9 => {
                if self.ip + 2 > self.commands.len() {
                    println!("Outside memory");
                    return r;
                }
                let param1 = self.get_parameter(self.ip + 1, mode1);
                self.relative_base += param1;
                self.ip += 2;
            },

            99 => {
                println!("{} - Halting", self.count);
                return Some(0xdeabbeff);
            },
            _ => println!("Invalid command: {}, ip: {}, commands: {:?}", self.commands[self.ip], self.ip, self.commands),
        }
        return r;
    }
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

    commands.resize(10000, 0);
    let mut emulator = IntcodeEmulator::new(commands, Intcode::new());

    loop {
        let r = emulator.emulate();
        if r == Some(0xdeabbeff) {
            break;
        }
    }
}
