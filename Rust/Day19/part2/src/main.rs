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
    o_x : usize,
    x : usize,
    y : usize,
    counter : usize,
    zero_found : bool,
    square_size : usize
}

impl Intcode {
    fn new() -> Intcode {
        Intcode {
            map : vec![Vec::new()],
            o_x : 0,
            x : 0,
            y : 0,
            counter : 0,
            zero_found : false,
            square_size : 0
        }
    }

    fn read_input(&mut self, _count : i64) -> i64 {
        let v : i64;
        if self.counter % 2 == 0 {
            v = self.x as i64;
            // print!("({}, ", v);
        } else {
            v = self.y as i64;
            // print!("{}) = ", v);
        }
        self.counter += 1;

        v
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        // println!("{}", value);
        self.map.last_mut().unwrap().push(value as i8);
        if value != 1 {
            self.zero_found = true;
        }       

        self.x += 1;
        if self.x == self.o_x + self.square_size {
            self.map.push(Vec::new());
            self.y += 1;
            self.x = self.o_x;
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
                // println!("{} - Halting", count);
                return commands[0];
            },
            _ => println!("Invalid command: {}, ip: {}, commands: {:?}", commands[ip], ip, commands),
        }
    }
    return 0;
}

fn improve_y(intcode : &mut Intcode, commands : &Vec<i64>) -> bool {
    let mut improve = false;
    loop {
        let mut ic = intcode.clone();
        for _i in 0..intcode.square_size * intcode.square_size {
            let mut c = commands.clone();
            emulate(&mut c, &mut ic);

            if ic.zero_found {
                break;
            }
        }

        if !ic.zero_found {
            println!("improve y: ({},{}) = {}", intcode.x, intcode.y, intcode.x * 10_000 + intcode.y);
            improve = true;
        } else {
            break;
        }

        intcode.y -= 1;
    }
    improve
}

fn improve_x(intcode : &mut Intcode, commands : &Vec<i64>) -> bool {
    let mut improve = false;
    loop {
        let mut ic = intcode.clone();
        for _i in 0..intcode.square_size * intcode.square_size {
            let mut c = commands.clone();
            emulate(&mut c, &mut ic);

            if ic.zero_found {
                break;
            }
        }

        if !ic.zero_found {
            println!("improve x: ({},{}) = {}", intcode.x, intcode.y, intcode.x * 10_000 + intcode.y);
            improve = true;
        } else {
            break;
        }

        intcode.x -= 1;
        intcode.o_x -= 1;
    }
    improve
}

fn improve_xy(intcode : &mut Intcode, commands : &Vec<i64>) -> bool {
    let mut improve = false;
    loop {
        let mut ic = intcode.clone();
        for _i in 0..intcode.square_size * intcode.square_size {
            let mut c = commands.clone();
            emulate(&mut c, &mut ic);

            if ic.zero_found {
                break;
            }
        }

        if !ic.zero_found {
            println!("improve xy: ({},{}) = {}", intcode.x, intcode.y, intcode.x * 10_000 + intcode.y);
            improve = true;
        } else {
            break;
        }

        intcode.x -= 1;
        intcode.o_x -= 1;
        intcode.y -= 1;
    }
    improve
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

    // 1 ->  0, 0   8,13
    // 2 ->  8,13   6,10
    // 3 -> 14,23   8,13
    // 4 -> 22,36   6,10
    // 5 -> 28,46
    // 6 -> 36,59

    const TARGET : usize = 100;

    // let origin_x = (TARGET / 2 - 1) * 14;
    // let origin_y = (TARGET / 2 - 1) * 23;
    let origin_x = 668;
    let origin_y = 1099;
    commands.resize(1000, 0);
    let mut intcode = Intcode::new();
    intcode.square_size = TARGET;
    intcode.x = origin_x;
    intcode.o_x = origin_x;
    intcode.y = origin_y;

    let mut improve = true;
    while improve {
        improve = false;

        improve |= improve_y(&mut intcode, &commands);
        intcode.y += 1;

        improve |= improve_x(&mut intcode, &commands);
        intcode.x += 1;
        intcode.o_x += 1;

        improve |= improve_xy(&mut intcode, &commands);
        intcode.y += 1;
        intcode.x += 1;
        intcode.o_x += 1;

        let mut out = false;
        for x in intcode.x - 10..intcode.x {
            for y in intcode.y - 10..intcode.y {
                let mut ic = intcode.clone();
                ic.x = x;
                ic.o_x = x;
                ic.y = y;
                for _i in 0..intcode.square_size * intcode.square_size {
                    let mut c = commands.clone();
                    emulate(&mut c, &mut ic);

                    if ic.zero_found {
                        break;
                    }
                }

                if !ic.zero_found {
                    intcode.x = x;
                    intcode.o_x = x;
                    intcode.y = y;
                    out = true;
                    improve = true;
                }
            }
            if out {
                break;
            }
        }
    }

    println!("Final Solution part2: ({},{}) = {}", intcode.x, intcode.y, intcode.x * 10_000 + intcode.y);    
    // println!("Origin: ({},{})", origin_x, origin_y);
    // print_map(&intcode.map);
}
