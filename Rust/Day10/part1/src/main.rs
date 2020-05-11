use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[warn(dead_code)]
fn print_map_ex(map : &Vec<Vec<u8>>, goodness : &Vec<usize>) {
    let mut counter = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 1 {
                print!("{}", goodness[counter]);
                counter += 1;
            } else {
                print!("{}", '.');
            }
        }
        println!();
    }
}

fn print_map(map : &Vec<Vec<u8>>, best_asteroid : &(usize, usize)) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", if map[i][j] == 1 { if j == best_asteroid.0 && i == best_asteroid.1 { 'X' } else { '#' } } else { '.' });
        }
        println!();
    }
}

//      y2 - y1
// m = ---------
//      x2 - x1

// y - y1 = m * (x - x1)
// y = m * x - m * x1 + y1
// y = m * x + y1 - m * x1
// 
// y1 - m_num * x1
//      ----------
//         m_den
//
// y1 * m_den - m_num * x1
// -----------------------
//         m_den

#[derive(Eq, Hash, Debug)]
enum Line {
    Vertical(i32),
    Horizontal(i32),
    WithSlop(i32, i32),
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Line::Vertical(x), &Line::Vertical(y)) => x == y,
            (&Line::Horizontal(x), &Line::Horizontal(y)) => x == y,
            (&Line::WithSlop(x1, x2), &Line::WithSlop(y1, y2)) => x1 == y1 && x2 == y2,
            _ => false
        }
    }
}

fn gcd(a: &i32, b: &i32) -> i32 {
    let mut _a = *a;
    let mut _b = *b;
    while _b != 0 {
        let tmp = _a;
        _a = _b;
        _b = tmp % _b;
    }
    _a
}

fn get_line(pp1 : (usize, usize), pp2 : (usize, usize)) -> Line {
    let p1 = (pp1.0 as i32, pp1.1 as i32);
    let p2 = (pp2.0 as i32, pp2.1 as i32);

    if p1.0 == p2.0 {
        return Line::Vertical((p2.1 - p1.1) / (p2.1 - p1.1).abs());
    }
    if p1.1 == p2.1 {
        return Line::Horizontal((p2.0 - p1.0) / (p2.0 - p1.0).abs());
    }

    let mut m_num = p2.1 - p1.1;
    let mut m_den = p2.0 - p1.0;
    let m_d = gcd(&m_num.abs(), &m_den.abs());
    m_num /= m_d;
    m_den /= m_d;
    if m_num == 0 {
        m_den = 1;
    }
    return Line::WithSlop(m_num, m_den);
}

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map : Vec<Vec<u8>> = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.trim().len() == 0 {
            break;
        }
        // Show the line and its number.
        map.push(line.as_bytes().to_vec());
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            map[i][j] = if map[i][j] == ('#' as u8) { 1 } else { 0 };
        }
    }

    let mut points : Vec<(usize, usize)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 1 {
                points.push((j, i));
            }
        }
    }

    let mut points_goodness : Vec<usize> = Vec::new();
    let mut max_i = points.len() + 1;
    let mut max_count = 0;
    
    for i in 0..points.len() {
        let mut lines : HashSet<Line> = HashSet::new();
        for j in 0..points.len() {
            if i != j {
                lines.insert(get_line(points[i], points[j]));
            }
        }
        if lines.len() > max_count {
            max_count = lines.len();
            max_i = i;
        }
    }

    // println!("{:?}", lines_from_point);

    println!("best asteroid point: {:?}, index: {}, with count: {}", points[max_i], max_i, max_count);
    print_map(&map, &points[max_i]);
    // println!();
    // println!();
    // print_map_ex(&map, &points_goodness);
}
