use std::io::{self,Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAP_ROW : usize = 40;
const MAP_COL : usize = 28;

fn print_board(board : &[u8], score : usize) -> (usize, usize) {
    let mut ball_pos = (0, 0);
    for c in 0..MAP_COL {
        for r in 0..MAP_ROW {        
            let cc = match board[r*MAP_COL + c] {
                0 => ' ',
                1 => if c == 0 { '_' } else { '|' },
                2 => { '#' },
                3 => 'T',
                4 => { ball_pos.0 = c; ball_pos.1 = r; '*' },
                _ => unreachable!()
            };
            print!("{}", cc);
        }
        println!();
    }
    println!("Score: {}", score);
    return ball_pos;
}

struct Intcode {
    map : Vec<u8>,
    output_pos : (usize, usize),
    output_count : usize,
    score_came : bool,
    score : usize,
    ball_pos : (usize, usize),
    paddle_pos : (usize, usize)
}

impl Intcode {
    fn new() -> Intcode {
        Intcode {
            map : vec![0u8; MAP_ROW * MAP_COL],
            output_pos : (0, 0),
            ball_pos : (0, 0),
            output_count : 0,
            score_came : false,
            score : 0,
            paddle_pos : (0, 0)
        }
    }

    fn read_input(&mut self, _count : i64) -> i64 {
        let ball_pos = print_board(&self.map, self.score);
        let diff_y = ball_pos.1 as i32 - self.ball_pos.1 as i32;

        // print!("{} - Input: ", _count);
        // io::stdout().flush().ok().expect("Could not flush stdout");

        // println!("diff: {}, prev ball pos: {:?}, new_ball_pos: {:?}, paddle pos: {:?}", diff_y, self.ball_pos, ball_pos, self.paddle_pos);

        // let mut input_text = String::new();
        // io::stdin()
        //     .read_line(&mut input_text)
        //     .expect("failed to read from stdin");

        let m;
        if ball_pos.1 < self.paddle_pos.1 {
            if diff_y > 0 {
                m = 0;
            } else {
                m = -1;
            }
        } else if ball_pos.1 > self.paddle_pos.1 {
            if diff_y < 0 {
                m = 0;
            } else {
                m = 1;
            }
        } else {
            if ball_pos.0 + 1 == self.paddle_pos.0 {
                m = 0;
            } else if diff_y > 0 {
                m = 1;
            } else if diff_y < 0 {
                m = -1;
            } else {
                m = 0;
            }
        }

        // println!("move: {}", m);
        self.ball_pos = ball_pos;
        return m;
    }

    fn write_output(&mut self, _count: i64, value : i64) {
        match self.output_count % 3 {
            0 => {
                if value == -1 {
                    self.score_came = true;
                } else {
                    self.output_pos.0 = value as usize;
                }
            },
            1 => {
                if !self.score_came || value != 0 {
                    self.output_pos.1 = value as usize;
                    self.score_came = false;
                }
            },
            2 => { 
                if self.score_came {
                    self.score = value as usize;
                    self.score_came = false;
                } else {
                    self.map[self.output_pos.0 * MAP_COL + self.output_pos.1] = value as u8;
                    if value == 3 {
                        self.paddle_pos.0 = self.output_pos.1;
                        self.paddle_pos.1 = self.output_pos.0;
                    }
                }
            },
            _ => unreachable!()
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

    commands.resize(3000, 0);
    commands[0] = 2;
    let mut intcode = Intcode::new();
    emulate(&mut commands, &mut intcode);

    println!("Solution: {}", intcode.score);
}
