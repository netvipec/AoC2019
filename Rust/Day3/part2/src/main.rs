use std::fs::File;
use std::io::Read;

fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Point {
            x: 0,
            y: 0,
        }
    }
}

fn intercept(p1a : &(Point, char, i32), p1b : &(Point, char, i32), p2a : &(Point, char, i32), p2b : &(Point, char, i32)) -> Option<(Point, i32)> {
    if (p1b.1 == 'L' || p1b.1 == 'R') &&
       (p2b.1 == 'L' || p2b.1 == 'R') {
        return None;
    }
    if (p1b.1 == 'U' || p1b.1 == 'D') &&
       (p2b.1 == 'U' || p2b.1 == 'D') {
        return None;
    }

    // println!("{:?} -> {:?}, {:?} -> {:?}", p1a, p1b, p2a, p2b);
    
    let mut pp1a = p1a;
    let mut pp1b = p1b;
    let mut pp2a = p2a;
    let mut pp2b = p2b;

    if p1b.1 == 'L' || p1b.1 == 'U' {
        pp1a = p1b;
        pp1b = p1a;
    }
    if p2b.1 == 'L' || p2b.1 == 'U' {
        pp2a = p2b;
        pp2b = p2a;
    }

    if p1b.1 == 'L' || p1b.1 == 'R' {
        if pp1a.0.x < pp2a.0.x && pp2a.0.x < pp1b.0.x &&
           pp2a.0.y < pp1a.0.y && pp1a.0.y < pp2b.0.y {
            let mut p : Point = Point::new();
            p.x = pp2a.0.x;
            p.y = pp1a.0.y;
            let cost = p1a.2 + p2a.2 + (p.x - p1a.0.x).abs() + (p.y - p2a.0.y).abs();
            // println!("{}", cost);
            return Some((p, cost));
        }
    } else {
        if pp2a.0.x < pp1a.0.x && pp1a.0.x < pp2b.0.x &&
           pp1a.0.y < pp2a.0.y && pp2a.0.y < pp1b.0.y {
            let mut p : Point = Point::new();
            p.x = pp1a.0.x;
            p.y = pp2a.0.y;
            let cost = p1a.2 + p2a.2 + (p.x - p2a.0.x).abs() + (p.y - p1a.0.y).abs();
            // println!("{}", cost);
            return Some((p, cost));
        }
    }

    return None;
}

fn main() {
    let filename = "../part1/src/input";
    let lines = lines_from_file(filename);

    let mut paths : Vec<Vec<String>> = Vec::new();

    for line in lines {
        let commands = line.split(",").map(|s| s.to_string()).collect();
        paths.push(commands);
    }

    let mut point_paths : Vec<Vec<(Point, char, i32)>> = Vec::new();

    for p in paths {
        let mut actual_point = Point::new();
        point_paths.push(vec![(actual_point, ' ', 0)]);
        let actual_path_index = point_paths.len() - 1;
        let mut cost = 0;
        for c in p {
            let cu0 = c.chars().next().unwrap();
            
            let parameter_str : String = c.chars().skip(1).collect();
            let parameter : i32 = parameter_str.parse().unwrap();
            cost += parameter;
            
            match cu0  {
                'R' => actual_point.x += parameter,
                'L' => actual_point.x -= parameter,
                'U' => actual_point.y -= parameter,
                'D' => actual_point.y += parameter,
                _   => panic!("Invalid command, {}", c),
            }

            point_paths[actual_path_index].push((actual_point, cu0, cost));
        }

        if actual_path_index == 1 {
            break;
        }
    }

    let mut interceptions : Vec<(Point, i32)> = Vec::new();

    for x in 0..point_paths[0].len()-1 {
        for y in 0..point_paths[1].len()-1 {
            match intercept(&point_paths[0][x], &point_paths[0][x+1], &point_paths[1][y], &point_paths[1][y+1]) {
                Some(x) => interceptions.push(x),
                None => continue,
            }
        }
    }

    interceptions.sort_by_key(|item| item.1);
    if interceptions.len() > 0 {
        println!("Solution {:?}", interceptions[0].1);
    } else {
        println!("No Solution");
    }
    // println!("{:?}", interceptions);
}