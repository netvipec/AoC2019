use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::cmp::min;

const D : [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn found_portal_exit(map : &Vec<Vec<char>>, row : usize, col : usize) -> Option<(usize, usize)> {
    for d in D.iter() {
        let r = row as i32 + d.0;
        let c = col as i32 + d.1;

        if r < 0 || c < 0 {
            continue;
        }

        let rr = r as usize;
        let cc = c as usize;
        if rr >= map.len() || cc >= map[rr].len() {
            continue;
        }

        if map[rr][cc] == '.' {
            return Some((rr, cc));
        }
    }
    None
}

fn found_neighborn_letter(map : &Vec<Vec<char>>, row : usize, col : usize) -> (usize, usize) {
    for d in D.iter() {
        let r = row as i32 + d.0;
        let c = col as i32 + d.1;

        if r < 0 || c < 0 {
            continue;
        }

        let rr = r as usize;
        let cc = c as usize;
        if rr >= map.len() || cc >= map[rr].len() {
            continue;
        }

        if 'A' <= map[rr][cc] && map[rr][cc] <= 'Z' {
            return (rr, cc);
        }
    }
    unreachable!()
}

fn find_route(map : &Vec<Vec<char>>, origin : &(usize, usize), destination : &(usize, usize), portals_move : &HashMap<(usize, usize), ((usize, usize), i32)>) -> usize {
    let mut points : HashSet<((usize, usize), i32)> = HashSet::new();
    let mut new_points : HashSet<((usize, usize), i32)> = HashSet::new();
    let mut visit_points : HashSet<((usize, usize), i32)> = HashSet::new();

    points.insert((origin.clone(), 0));
    visit_points.insert((origin.clone(), 0));

    let mut route_size = 0;
    loop {
        route_size += 1;

        let mut have_portal_move = false;

        for p in points.iter() {
            for (dx, dy) in D.iter() {
                let new_xx = (p.0).0 as i32 + dx;
                let new_yy = (p.0).1 as i32 + dy;
                if new_xx < 0 || new_yy < 0 {
                    continue;
                }                   
                let new_x = new_xx as usize;
                let new_y = new_yy as usize;
                if new_x >= map.len() || new_y >= map[new_x].len() {
                    continue;
                }

                let k = ((new_x, new_y), p.1);
                if map[new_x][new_y] == '.' && !visit_points.contains(&k) {
                    visit_points.insert(k.clone());
                    new_points.insert(k);
                }
            }

            if portals_move.contains_key(&p.0) {
                let other_end = portals_move.get(&p.0).unwrap();

                if p.1 + other_end.1 >= 0 {
                    let new_level = (other_end.0.clone(), p.1 + other_end.1);

                    if !visit_points.contains(&new_level) {
                        visit_points.insert(new_level.clone());
                        new_points.insert(new_level);
                        have_portal_move = true;
                    }
                }
            }
        }

        if have_portal_move {
            // println!("pm: {:?}", new_points);
        }

        match new_points.iter().position(|&a| a == (*destination, 0)) {
            Some(_) => return route_size,
            None => {}
        }

        points.clear();
        points = new_points.clone();
        new_points = HashSet::new();
    }
}

fn main() {
    // let filename = "src/input0";
    let filename = "../part1/src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map : Vec<Vec<char>> = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.trim().len() == 0 {
            break;
        }

        map.push(line.chars().collect());
    }

    let mut portals : Vec<(String, Vec<(usize, usize)>)> = Vec::new();
    let mut portals_move : HashMap<(usize, usize), ((usize, usize), i32)> = HashMap::new();
    
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if 'A' <= map[r][c] && map[r][c] <= 'Z' {
                let (r1,c1) = found_neighborn_letter(&map, r, c);
                let mut s = String::new();
                if map[r][c] <= map[r1][c1] {
                    s.push(map[r][c]);
                    s.push(map[r1][c1]);
                } else  {
                    s.push(map[r1][c1]);
                    s.push(map[r][c]);
                }

                let idx = match portals.iter().position(|a| *a.0 == s) {
                    Some(x) => x,
                    None => { portals.push((s.clone(), Vec::new())); portals.len() - 1 }
                };

                let (or, oc) = match found_portal_exit(&map, r, c) {
                    Some(x) => x,
                    None => found_portal_exit(&map, r1, c1).unwrap()
                };

                if (s != "AA" && s != "ZZ") || portals[idx].1.len() < 1 {
                    match portals[idx].1.iter().position(|&a| a == (or, oc)) {
                        Some(_) => {},
                        None => portals[idx].1.push((or, oc))
                    };
                }
            }
        }
    }

    let mut origin : (usize, usize) = (0, 0);
    let mut destination : (usize, usize) = (0, 0);
    for p in portals.iter() {
        if p.0 == "AA" {
            origin = p.1[0];
            continue;
        }
        if p.0 == "ZZ" {
            destination = p.1[0];
            continue;
        }
        if p.1.len() != 2 {
            unreachable!();
        }

        let min0 = min(p.1[0].0, min(p.1[0].1, min(map.len() - p.1[0].0, map[p.1[0].0].len() - p.1[0].1)));
        let min1 = min(p.1[1].0, min(p.1[1].1, min(map.len() - p.1[1].0, map[p.1[1].0].len() - p.1[1].1)));
        portals_move.insert(p.1[0], (p.1[1], if min0 < min1 { -1 } else {  1 }));
        portals_move.insert(p.1[1], (p.1[0], if min0 < min1 {  1 } else { -1 }));
    }

    println!("Size hxw: {}x{}", map.len(), map[0].len());
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }

    println!("portals: {:?}", portals);
    println!("portals move: {:?}", portals_move);
    println!("origin: {:?}", origin);
    println!("destination: {:?}", destination);

    let route_size = find_route(&map, &origin, &destination, &portals_move);

    println!("Solution part1: {}", route_size);
}
