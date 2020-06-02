extern crate itertools;

use itertools::Itertools;
use std::io::{self,Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_SIZE_CMD : usize = 100;

#[derive(Clone)]
struct Intcode {
    last_cmd : String,
    last_cmd_idx : usize,

    commands_to_check : Vec<String>,
    commands_to_check_idx : usize,
    objects : Vec<String>,
    objects_dropped : bool,
    objects_put : bool
}

impl Intcode {
    fn new() -> Intcode {
        Intcode {
            last_cmd : String::new(),
            last_cmd_idx : MAX_SIZE_CMD,
            commands_to_check : vec!["east".to_string(), "take manifold".to_string(), "south".to_string(), "take whirled peas".to_string(), 
                                     "north".to_string(), "west".to_string(), "south".to_string(), "take space heater".to_string(), "east".to_string(),
                                     "east".to_string(), "take bowl of rice".to_string(), "north".to_string(), "take klein bottle".to_string(), 
                                     "north".to_string(), "take spool of cat6".to_string(), "south".to_string(), "south".to_string(), "west".to_string(),
                                     "west".to_string(), "south".to_string(), "take dark matter".to_string(), "north".to_string(), "east".to_string(),
                                     "north".to_string(), "west".to_string(), "south".to_string(), "take antenna".to_string(), "north".to_string(),
                                     "east".to_string(), "south".to_string(), "east".to_string(), "north".to_string(), "north".to_string(), "west".to_string()],
            
            commands_to_check_idx : 0,
            objects : vec!["manifold".to_string(),
                           "whirled peas".to_string(),
                           "space heater".to_string(),
                           "bowl of rice".to_string(),
                           "klein bottle".to_string(),
                           "spool of cat6".to_string(),
                           "dark matter".to_string(),
                           "antenna".to_string()],
            objects_dropped : false,
            objects_put : false
        }
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
            if self.commands_to_check_idx < self.commands_to_check.len() {
                self.last_cmd = self.commands_to_check[self.commands_to_check_idx].clone();
                self.last_cmd_idx = 1;
                self.commands_to_check_idx += 1;
            } else {
                if !self.objects_dropped {
                    self.objects_dropped = true;
                    let d = String::from("drop ");
                    for o in self.objects.iter() {
                        self.commands_to_check.push(d.clone() + o);
                    }

                    self.last_cmd = self.commands_to_check[self.commands_to_check_idx].clone();
                    self.last_cmd_idx = 1;
                } else {
                    if !self.objects_put {
                        self.objects_put = true;

                        let t = String::from("take ");
                        let d = String::from("drop ");
                        let comb : Vec<_> = self.objects.iter().combinations(4).collect();
                        for c in comb.iter() {
                            for o in c.iter() {
                                self.commands_to_check.push(t.clone() + o);
                            }
                            self.commands_to_check.push("north".to_string());
                            for o in c.iter() {
                                self.commands_to_check.push(d.clone() + o);
                            }
                        }

                        self.last_cmd = self.commands_to_check[self.commands_to_check_idx].clone();
                        self.last_cmd_idx = 1;
                    } else {
                        // manual input
                        print!("{} - Input: ", _count);
                        io::stdout().flush().ok().expect("Could not flush stdout");

                        let mut input_text = String::new();
                        io::stdin()
                            .read_line(&mut input_text)
                            .expect("failed to read from stdin");

                        self.last_cmd = input_text.trim().to_string();
                        self.last_cmd_idx = 1;
                    }
                }
            }
            let c = *self.last_cmd.as_bytes().get(0).unwrap();
            print!("Input send: {}", c as char);
            return c as i64;
        }
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        if 0 <= value && value < 256 {
            let c = (value as u8) as char;
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
