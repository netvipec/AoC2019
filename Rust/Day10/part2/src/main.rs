use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;

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

fn print_map(map : &Vec<Vec<u8>>, best_asteroid : &(usize, usize), vaporized : &(usize, usize)) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let c = if map[i][j] == 1 {
                        if j == best_asteroid.0 && i == best_asteroid.1 { 'X' } 
                        else if j == vaporized.0 && i == vaporized.1 { '*' }
                        else { '#' } 
                    } else { '.' };
            print!("{}", c);
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
        return Line::Vertical((p1.1 - p2.1) / (p1.1 - p2.1).abs());
    }
    if p1.1 == p2.1 {
        return Line::Horizontal((p2.0 - p1.0) / (p2.0 - p1.0).abs());
    }

    let mut m_num = p1.1 - p2.1;
    let mut m_den = p2.0 - p1.0;
    let m_d = gcd(&m_num.abs(), &m_den.abs());
    m_num /= m_d;
    m_den /= m_d;
    if m_num == 0 {
        m_den = 1;
    }
    return Line::WithSlop(m_num, m_den);
}

fn angle(l : &Line) -> f64 {
    match l {
        &Line::Vertical(a) => if a > 0 { 0.0 } else { 180.0 },
        &Line::Horizontal(a) => if a > 0 { 90.0 } else { 270.0 },
        &Line::WithSlop(an, ad) => {
            let mut ang = ((an as f64)/(ad as f64)).atan() * 180.0 / std::f64::consts::PI;
            if ad < 0 && an < 0 {   // quadrant III
                ang = 270.0 - ang;
            } else if ad < 0 {      // quadrant II
                ang = 270.0 + ang.abs(); // it actually substracts
                println!("{:?} -> {}", l, ang);
            } else if an < 0 {      // quadrant IV
                ang = 90.0 + ang.abs(); // it actually substracts
            } else {                // quadrant I
                ang = 90.0 - ang;
            }
            ang
        },
    }
}

fn line_compare(l1 : &Line, l2 : &Line) -> Ordering {
    let l1_angle = angle(l1);
    let l2_angle = angle(l2);

    // println!("ang1l1: {:?} -> l2: {:?}", l1, l2);
    // println!("ang1: {} -> ang2: {}", l1_angle, l2_angle);

    l1_angle.partial_cmp(&l2_angle).unwrap()
}

fn distance(p1 : &(usize, usize), p2 : &(usize, usize)) -> usize {
    let a = p2.0 as i32 - p1.0 as i32;
    let b = p2.1 as i32 - p1.1 as i32;

    return (a * a + b * b) as usize;
}

fn get_vaporized(points : &Vec<(usize, usize)>, laser_idx : usize, n_vaporized : usize) -> (usize, usize) {
    let mut lines : HashMap<Line, Vec<usize>> = HashMap::new();
    for j in 0..points.len() {
        if laser_idx != j {
            let line = get_line(points[laser_idx], points[j]);
            if lines.contains_key(&line) {
                lines.get_mut(&line).unwrap().push(j);
            } else {
                lines.insert(line, vec![j]);
            }
        }
    }

    // println!("unsorted: {:?}", lines);

    for (line, ps) in &mut lines {
        if ps.len() == 1 {
            continue;
        }
        ps.sort_by(|a, b| {
            distance(&points[laser_idx], &points[*a]).cmp(&distance(&points[laser_idx], &points[*b]))
        });

        // println!("{} -> {:?}", angle(line), line);
    }

    // println!("unsorted: {:?}", lines);

    let mut best_lines = lines.iter().collect::<Vec<_>>();
    best_lines.sort_by(|a, b| {
        return line_compare(a.0, b.0);
    });

    // for i in 0..points.len() {
    //     println!("{} -> {:?}", i, points[i]);
    // }
    // println!("sorted: {:?}", best_lines);

    let mut counter = 0;
    let mut visited_points : Vec<usize> = vec![0; best_lines.len()];
    loop {
        let mut modified = false;
        println!("Begin");
        for i in 0..best_lines.len() {
            if visited_points[i] < best_lines[i].1.len() {
                counter += 1;
                println!("{} asteroid vaporized is {:?}", counter, points[best_lines[i].1[visited_points[i]]]);
                if counter == n_vaporized {
                    return points[best_lines[i].1[visited_points[i]]];
                }

                visited_points[i] += 1;
                modified = true;
            }
        }
        println!("End");

        if !modified {
            break;
        }
    }

    return (12345, 12345);
}

fn main() {
    // let filename = "../part1/src/input0";
    let filename = "../part1/src/input";
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

    let vaporized_n = get_vaporized(&points, max_i, 200);

    println!("best asteroid point: {:?}, index: {}, with count: {}", points[max_i], max_i, max_count);
    println!("the {}nth vaporized asteroid would be: {:?} -> {}", 200, vaporized_n, vaporized_n.0 * 100 + vaporized_n.1);
    print_map(&map, &points[max_i], &vaporized_n);
    
    // println!();
    // println!();
    // print_map_ex(&map, &points_goodness);
}
