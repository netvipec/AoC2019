use std::fs::File;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Intcode {
    messages : VecDeque<i64>,
    addr : i64,
    addr_send : bool,
    to_addr : (i64, i64, i64),
    message_counter : i32,
    read : bool,
    bad_read_count : i64
}

impl Intcode {
    fn new(addr : i64) -> Intcode {
        Intcode {
            messages : VecDeque::new(),
            addr : addr,
            addr_send : false,
            to_addr : (-1, -1, -1),
            message_counter : 0,
            read : false,
            bad_read_count : 0
        }
    }

    fn read_input(&mut self, _count : i64) -> i64 {
        if !self.addr_send {
            self.addr_send = true;
            return self.addr;
        }

        self.read = true;
        if self.messages.len() == 0 {
            self.bad_read_count += 1;
            return -1;
        } else {
            let v = self.messages.pop_front().unwrap();
            self.bad_read_count = 0;
            return v;
        }
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        if self.message_counter == 3 {
            self.message_counter = 0;
        }
        match self.message_counter {
            0 => self.to_addr = (value, -1, -1),
            1 => self.to_addr.1 = value,
            2 => self.to_addr.2 = value,
            _ => {}
        };
        self.message_counter += 1;
        self.bad_read_count = 0;
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
                return r;
            },
            _ => println!("Invalid command: {}, ip: {}, commands: {:?}", self.commands[self.ip], self.ip, self.commands),
        }
        return r;
    }
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

    commands.resize(3000, 0);

    let mut network = Vec::new();
    for i in 0..50 {
        network.push(IntcodeEmulator::new(commands.clone(), Intcode::new(i)));
    }

    let mut nat_message : (i64, i64, i64) = (-1,-1,-1);
    let mut y_sended : (i64, i64, i64) = (-1,-1,-1);

    loop {
        let mut messages_list_size : Vec<(usize, usize)> = (0..).zip(network.iter().map(|c| c.intcode.messages.len())).collect();
        messages_list_size.sort_by(|a, b| b.1.cmp(&a.1));
        // println!("message list size: {:?}", messages_list_size);

        for pci in 0..messages_list_size.len() {
            let pc = messages_list_size[pci].0;
            loop {
                network[pc].emulate();
                if network[pc].intcode.message_counter == 3 {
                    network[pc].intcode.message_counter = 0;
                    let m = network[pc].intcode.to_addr;
                    if m.0 == 255 {
                        if nat_message == m {
                            // println!("send message to nat: {:?}, REPEATED from {}", nat_message, pc);
                        } else {
                            println!("send message to nat: {:?}, UPDATED from {}", m, pc);
                            nat_message = m.clone();
                        }
                    } else {
                        network[m.0 as usize].intcode.messages.push_back(m.1);
                        network[m.0 as usize].intcode.messages.push_back(m.2);
                        // println!("send message to addr: {:?}, from {}", m, pc);
                    }

                    break;
                }
                if network[pc].intcode.read {
                    network[pc].intcode.read = false;
                    if network[pc].intcode.bad_read_count > 0 {
                        break;
                    }
                }
            }
        }

        let pc_in_read = network.iter().filter(|&c| c.intcode.messages.len() == 0 && c.intcode.bad_read_count > 5).count();
        if pc_in_read == network.len() {
            if y_sended.2 == nat_message.2 {
                println!("Solution: {:?}", nat_message);
                return;
            }
            network[0].intcode.messages.push_back(nat_message.1);
            network[0].intcode.messages.push_back(nat_message.2);
            y_sended = nat_message;

            println!("Send to addr0 message: {:?}", nat_message);
        }
    }
}
