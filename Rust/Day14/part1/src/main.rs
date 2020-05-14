use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn insert_type_value(type_idx : &mut Vec<String>, data : &str) -> (usize, i64) {
    let mut vt = data.trim().split(" ");        
    let v : i64 = vt.next().unwrap().parse().unwrap();
    let t = vt.next().unwrap();

    let t_idx = match type_idx.iter().position(|r| r.eq(t)) {
        Some(i) => i,
        None => { type_idx.push(t.to_string()); type_idx.len()-1 }
    };
    
    return (t_idx, v);
}

fn merge(all_dep : &mut Vec<HashMap<usize, i64>>, dep : HashMap<usize, i64>) {
    if all_dep.len() == 0 {
        all_dep.push(dep);
        return;
    }

    for ad in all_dep.iter_mut() {
        for d in dep.iter() {
            *ad.entry(*d.0).or_insert(0) += *d.1;
        }
    }
}

fn merge_one(all_dep : &mut Vec<HashMap<usize, i64>>, dep : &(usize, i64)) {
    if all_dep.len() == 0 {
        let mut t : HashMap<usize, i64> = HashMap::new();
        t.insert(dep.0, dep.1);
        all_dep.push(t);
        return;
    }

    for ad in all_dep.iter_mut() {
        *ad.entry(dep.0).or_insert(0) += dep.1;
    }
}

// (material, need, free)

fn add_new_solutions(actual_mat : &mut Vec<Vec<(usize, i64)>>, reactions : &Vec<((usize, i64), Vec<(usize, i64)>)>, mat : &Vec<(usize, i64)>, ore_idx : usize) {
    // println!("begin new sols: {:?}", mat);
    let mut all_dep : Vec<HashMap<usize, i64>> = Vec::new();
    for m in mat.iter() {
        if m.1 < 0 || m.0 == ore_idx {
            merge_one(&mut all_dep, m);
            continue;
        }
        // println!("{:?}", m);
        let mut m_reactions_idx = reactions.iter().position(|r| (r.0).0 == m.0).unwrap();
        let m_need = m.1;
        while m_reactions_idx < reactions.len() && (reactions[m_reactions_idx].0).0 == m.0 {
            let mut dep : HashMap<usize, i64> = HashMap::new();
            let times = m_need / (reactions[m_reactions_idx].0).1 + if m_need % (reactions[m_reactions_idx].0).1 != 0 { 1 } else { 0 };
            if times * (reactions[m_reactions_idx].0).1 - m_need > 0 {
                let remain = times * (reactions[m_reactions_idx].0).1 - m_need;
                *dep.entry(m.0).or_insert(0) -= remain;
            }
            for i in reactions[m_reactions_idx].1.iter() {
                *dep.entry(i.0).or_insert(0) += i.1 * times;
            }

            // println!("dep: {:?}", dep);

            merge(&mut all_dep, dep);

            // println!("all_dep: {:?}", all_dep);

            m_reactions_idx += 1;
        }
    }

    for ad in all_dep.iter() {
        let mut ad_vec = Vec::new();
        for i in ad.iter() {
            ad_vec.push((*i.0, *i.1));
        }
        actual_mat.push(ad_vec);
    }
}

fn solve(reactions : &Vec<((usize, i64), Vec<(usize, i64)>)>, fuel_idx : usize, ore_idx : usize) -> i64 {
    let mut actual_mat : Vec<Vec<(usize, i64)>> = Vec::new();
    actual_mat.push(vec![(fuel_idx, 1)]);

    let mut best_solution : i64 = i64::max_value();
    loop {
        if actual_mat.len() == 0 {
            return best_solution;
        }

        let mat = actual_mat.pop().unwrap();
        let need_count = mat.iter().filter(|&a| a.1 > 0).count();
        if need_count == 1 {
            match mat.iter().position(|&a| a.0 == ore_idx) {
                Some(i) => { 
                    // println!("{:?}", mat);
                    if mat[i].1 < best_solution {
                        best_solution = mat[i].1;
                    }
                    continue;
                },
                None => {}
            };
        }

        // println!("mat: {:?}", mat);

        add_new_solutions(&mut actual_mat, &reactions, &mat, ore_idx);

        // println!("actual_mat: {:?}", actual_mat);
    }
}

fn solve2(reactions : &Vec<((usize, i64), Vec<(usize, i64)>)>, fuel_idx : usize, ore_idx : usize, part1_solution : i64) -> i64 {
    let mut actual_mat : Vec<Vec<(usize, i64)>> = Vec::new();
    let ore_storage : i64 = 1_000_000_000_000;
    let mut target_fuel : i64 = 2 * ore_storage / part1_solution;

    println!("{}", target_fuel);
    actual_mat.push(vec![(fuel_idx, target_fuel)]);

    let mut best_solution : i64 = i64::max_value();
    loop {
        if actual_mat.len() == 0 {
            if best_solution < ore_storage {
                best_solution = i64::max_value();
                target_fuel += 1;
                // println!("{} {}", target_fuel, best_solution);
                actual_mat.push(vec![(fuel_idx, target_fuel)]);
            } else {
                return target_fuel - 1;
            }
        }

        let mat = actual_mat.pop().unwrap();
        let need_count = mat.iter().filter(|&a| a.1 > 0).count();
        if need_count == 1 {
            match mat.iter().position(|&a| a.0 == ore_idx) {
                Some(i) => { 
                    // println!("{:?}", mat);
                    if mat[i].1 < best_solution {
                        best_solution = mat[i].1;
                    }
                    continue;
                },
                None => {}
            };
        }

        // println!("mat: {:?}", mat);

        add_new_solutions(&mut actual_mat, &reactions, &mat, ore_idx);

        // println!("actual_mat: {:?}", actual_mat);
    }
}

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut type_idx = Vec::new();
    let mut reactions = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.trim().len() == 0 {
            break;
        }

        // Show the line and its number.
        let mut reaction = line.split("=>");

        let left = &reaction.next().unwrap();
        let right = &reaction.next().unwrap();

        // println!("{} {}", left, right);

        let produced = insert_type_value(&mut type_idx, right);
        let mut need_to_produce = Vec::new();

        for l in left.split(",") {
            need_to_produce.push(insert_type_value(&mut type_idx, l));
        }

        reactions.push((produced, need_to_produce));
    }

    reactions.sort_by(|a,b| a.0.cmp(&b.0));
    // println!("{:?}", type_idx);
    // println!("{:?}", reactions);

    let fuel_idx = type_idx.iter().position(|r| r == "FUEL").unwrap();
    let ore_idx = type_idx.iter().position(|r| r == "ORE").unwrap();

    // println!("Fuel idx: {}, Ore idx: {}", fuel_idx, ore_idx);

    let solution1 = solve(&reactions, fuel_idx, ore_idx);
    println!("Solution part1: {}", solution1);

    let solution2 = solve2(&reactions, fuel_idx, ore_idx, solution1);
    println!("Solution part2: {}", solution2);
}
