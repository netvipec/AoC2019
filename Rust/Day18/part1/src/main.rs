use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

const D : [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn insert_vpoint(visit_points : &mut HashMap<(usize, usize), HashSet<usize>>, 
                pos : &(usize, usize), 
                pos_keys : &HashSet<usize>) {
    if !visit_points.contains_key(&pos) {
        visit_points.insert(*pos, pos_keys.clone());
    } else {
        let v_p_k = visit_points.get(&pos).unwrap();
        let p_union = HashSet::from_iter(pos_keys.union(v_p_k).cloned());
        visit_points.insert(*pos, p_union);
    }
}

fn insert_point(new_points : &mut HashMap<(usize, usize), HashSet<usize>>, 
                visit_points : &mut HashMap<(usize, usize), HashSet<usize>>, 
                pos : &(usize, usize), 
                pos_keys : &HashSet<usize>, 
                new_key : usize) {
    let mut init_hs : HashSet<usize> = HashSet::new();
    for pkv in pos_keys.iter() {
        init_hs.insert(pkv | new_key);
    }

    if !visit_points.contains_key(&pos) {
        new_points.insert(*pos, init_hs.clone());
        visit_points.insert(*pos, init_hs);
    } else {
        let p_k = visit_points.get(&pos).unwrap();
        let p_diff_k = HashSet::from_iter(init_hs.difference(&p_k).cloned());
        if p_diff_k.len() > 0 {
            if !new_points.contains_key(&pos) {
                new_points.insert(*pos, p_diff_k.clone());
                insert_vpoint(visit_points, pos, &p_diff_k);
            } else {
                let n_p_k = new_points.get(&pos).unwrap();
                let p_union = HashSet::from_iter(p_diff_k.union(n_p_k).cloned());
                if p_union.len() > 0 {
                    new_points.insert(*pos, p_union.clone());
                    insert_vpoint(visit_points, pos, &p_union);
                }
            }
        }
    }
}

fn find_route(map : &Vec<Vec<char>>, origin : &(&(usize, usize), usize), keys : &HashMap<char, (usize, usize)>) -> usize {
    let mut points : HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    let mut new_points : HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    let mut visit_points : HashMap<(usize, usize), HashSet<usize>> = HashMap::new();

    let mut init_hs : HashSet<usize> = HashSet::new();
    init_hs.insert(0);

    points.insert(*origin.0, init_hs.clone());
    visit_points.insert(*origin.0, init_hs);

    let final_mask : usize = (1 << keys.len()) - 1;

    let mut counter = 0;
    loop {
        counter += 1;

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

                let k = (new_x, new_y);
                if map[new_x][new_y] == '.' || map[new_x][new_y] == '@' {
                    insert_point(&mut new_points, &mut visit_points, &k, p.1, 0);
                } else if 'a' <= map[new_x][new_y] && map[new_x][new_y] <= 'z' {
                    let new_key = 1 << ((map[new_x][new_y] as usize) - ('a' as usize));
                    insert_point(&mut new_points, &mut visit_points, &k, p.1, new_key);
                } else if 'A' <= map[new_x][new_y] && map[new_x][new_y] <= 'Z' {
                    let need_key = 1 << ((map[new_x][new_y] as usize) - ('A' as usize));
                    let mut new_keys : HashSet<usize> = HashSet::new();
                    for k in p.1.iter() {
                        if (k & need_key) > 0 {
                            new_keys.insert(*k);
                        }
                    }
                    if new_keys.len() > 0 {
                        insert_point(&mut new_points, &mut visit_points, &k, &new_keys, 0);
                    }
                }
            }
        }

        for np in new_points.iter() {
            if np.1.contains(&final_mask) {
                return counter;
            }
        }

        // println!("counter: {}", counter);
        // println!("new points: {:?}", new_points);
        // println!("visit points: {:?}", visit_points);

        points.clear();
        points = new_points.clone();
        new_points = HashMap::new();
    }
}

fn main() {
    let filename = "src/input";
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

    let mut doors : HashMap<char, (usize, usize)> = HashMap::new();
    let mut keys : HashMap<char, (usize, usize)> = HashMap::new();
    let mut origin : (usize, usize) = (0, 0);

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if 'a' <= map[r][c] && map[r][c] <= 'z' {
                keys.insert(map[r][c], (r, c));
            } else if 'A' <= map[r][c] && map[r][c] <= 'Z' {
                doors.insert(map[r][c], (r, c));
            } else if map[r][c] == '@' {
                origin = (r, c);
            }
        }
    }

    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }

    println!("doors: {:?}", doors);
    println!("keys: {:?}", keys);
    println!("origin: {:?}", origin);

    println!("route from {:?}, result: {:?}", origin, find_route(&map, &(&origin, 0), &keys));
}
