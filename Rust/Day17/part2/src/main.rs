use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

const D : [(i8, i8);4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn print_map(map : &Vec<Vec<i8>>) {
    println!("Map size: {}x{}", map.len(), map[0].len());
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let c = if map[r][c] == 35 { '#' } 
                    else if map[r][c] == 46 { '.' } 
                    else if map[r][c] == 60 { '<' } 
                    else if map[r][c] == 62 { '>' } 
                    else if map[r][c] == 94 { '^' } 
                    else if map[r][c] == 118 { 'v' } 
                    else { '0' };
            print!("{}", c);
        }
        println!();
    }
}

struct Intcode {
    map : Vec<Vec<i8>>,
    map_printed : bool,
    robot_pos : (usize, usize),
    DLR : HashMap<(usize, usize), char>,
    main_movement : Vec<char>,
    func_a : Vec<char>,
    func_b : Vec<char>,
    func_c : Vec<char>,
    idx1 : usize,
    idx2 : usize,
    feed_map : Vec<Vec<i8>>,
    feed_map_write : bool
}

impl Intcode {
    fn new() -> Intcode {
        let mut s = Intcode {
            map : vec![Vec::new()],
            map_printed : false,
            robot_pos : (0, 0),
            DLR : HashMap::new(),
            main_movement : Vec::new(),
            func_a : Vec::new(),
            func_b : Vec::new(),
            func_c : Vec::new(),
            idx1 : 0,
            idx2 : 0,
            feed_map : Vec::new(),
            feed_map_write : false,
        };
        s.DLR.insert((0, 2), 'L');
        s.DLR.insert((2, 0), 'R');
        s.DLR.insert((0, 3), 'R');
        s.DLR.insert((3, 0), 'L');
        s.DLR.insert((1, 2), 'R');
        s.DLR.insert((2, 1), 'L');
        s.DLR.insert((1, 3), 'L');
        s.DLR.insert((3, 1), 'R');
        return s;
    }

    fn find_direction(&self, pos : &(usize, usize), d : usize) -> ((usize, usize), (usize, usize)) {
        let inv_d = if d % 2 == 0 { d + 1 } else { d - 1 };
        for i in 0..D.len() {
            if i == inv_d {
                continue;
            }
            let new_x = (pos.0 as i8) + D[i].0;
            let new_y = (pos.1 as i8) + D[i].1;
            if new_x >= 0 && new_y >= 0 {
                let mut x = new_x as usize;
                let mut y = new_y as usize;

                if x < self.map.len() && y < self.map[x].len() && self.map[x][y] == 35 {
                    let mut len = 1;
                    let mut saved_x = x;
                    let mut saved_y = y;
                    while self.map[x][y] == 35 {
                        saved_x = x;
                        saved_y = y;

                        len += 1;

                        let new_x = (x as i8) + D[i].0;
                        let new_y = (y as i8) + D[i].1;
                        if new_x < 0 || new_y < 0 {
                            break;
                        }
                        x = new_x as usize;
                        y = new_y as usize;
                        if x >= self.map.len() || y >= self.map[x].len() {
                            break;
                        }
                    }
                    return ((i, len - 1), (saved_x, saved_y));
                }
            }
        }
        ((0, 0), (0, 0))
    }

    fn calculate_move_sequence(&self) -> String {
        println!("robot pos: {:?}", self.robot_pos);

        let mut sequence : String = String::new();

        let mut actual_robot_pos = self.robot_pos.clone();
        let mut d : usize = 0;

        loop {
            let (dl, pos) = self.find_direction(&actual_robot_pos, d);
            if dl.0 == 0 && dl.1 == 0 && pos.0 == 0 && pos.1 == 0 {
                println!("Sequence: {}", sequence);
                return sequence;
            }

            // println!("{:?} = {:?} -> {:?} {}", (d, dl.0), actual_robot_pos, pos, dl.1);
            let letter = self.DLR.get(&(d, dl.0)).unwrap().to_string();
            let command = letter + "," + &dl.1.to_string() + ",";
            sequence.push_str(&command);
            // println!("{}", sequence);

            actual_robot_pos = pos;
            d = dl.0;
        }  
    }

    fn read_input(&mut self, _count : i64) -> i64 {
        if !self.map_printed {
            println!("Printing map: ");
            self.map.remove(self.map.len() - 1);
            print_map(&self.map);
            self.map_printed = true;

            self.calculate_move_sequence();

            self.main_movement = vec!['A',',','A',',','B',',','C',',','B',',','C',',','B',',','C',',','A',',','C'];
            self.func_a =        vec!['R',',','6',',','L',',','8',',','R',',','8'];
            self.func_b =        vec!['R',',','4',',','R',',','6',',','R',',','6',',','R',',','4',',','R',',','4'];
            self.func_c =        vec!['L',',','8',',','R',',','6',',','L',',','1','0',',','L',',','1','0'];
        }

        return match self.idx1 {
            0 => {
                if self.idx2 == self.main_movement.len() {
                    self.idx1 += 1;
                    self.idx2 = 0;
                    println!("Send main movement");
                    10
                } else {
                    let v = self.main_movement[self.idx2] as i64;
                    self.idx2 += 1;
                    v
                }
            },
            1 => {
                if self.idx2 == self.func_a.len() {
                    self.idx1 += 1;
                    self.idx2 = 0;
                    println!("Send func a");
                    10
                } else {
                    let v = self.func_a[self.idx2] as i64;
                    self.idx2 += 1;
                    v
                }
            },
            2 => {
                if self.idx2 == self.func_b.len() {
                    self.idx1 += 1;
                    self.idx2 = 0;
                    println!("Send func b");
                    10
                } else {
                    let v = self.func_b[self.idx2] as i64;
                    self.idx2 += 1;
                    v
                }
            },
            3 => {
                if self.idx2 == self.func_c.len() {
                    self.idx1 += 1;
                    self.idx2 = 0;
                    println!("Send func c");
                    10
                } else {
                    let v = self.func_c[self.idx2] as i64;
                    self.idx2 += 1;                
                    v
                }
            },
            4 => {
                self.idx1 += 1;
                'N' as i64
            }
            5 => {
                println!("Send live feed: ");
                self.idx1 += 1;
                self.feed_map_write = true;
                10
            },
            _ => unreachable!()
        }
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        if !self.map_printed && self.map.len() <= 47 {
            if value == 10 {
                self.map.push(Vec::new());
            } else {
                self.map.last_mut().unwrap().push(value as i8);
                if value != 35 && value != 46 && value != 10 {
                    self.robot_pos.0 = self.map.len() - 1;
                    self.robot_pos.1 = self.map.last().unwrap().len() - 1;
                    println!("robot pos: {:?}, value: {}", self.robot_pos, value);
                }
            }
        } else {
            if self.feed_map_write {
                if value == 10 {
                    if self.feed_map.len() > 0 && self.feed_map.last().unwrap().len() == 0 {
                        self.feed_map.remove(self.feed_map.len() - 1);
                        print_map(&self.feed_map);
                        println!();

                        self.feed_map.clear();
                    }
                    self.feed_map.push(Vec::new());
                } else {
                    self.feed_map.last_mut().unwrap().push(value as i8);
                    if value != 35 && value != 46 && value != 60 && value != 62 && value != 94 && value != 118 {
                        println!("Output: {}", value);
                    } 
                }
            } else {
                let v = [value as u8];
                let v_str = std::str::from_utf8(&v).unwrap();
                if v_str.is_ascii() {
                    println!("Output: {}", v_str);
                } else {
                    println!("Output: {}", value);
                }
            }
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

    commands.resize(4000, 0);
    commands[0] = 2;
    let mut intcode = Intcode::new();
    emulate(&mut commands, &mut intcode);
}
