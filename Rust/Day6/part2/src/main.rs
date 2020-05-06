use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn get_origin_path(object_dependency : &Vec<i32>, idx : usize) -> Vec<i32> {
    let mut i = idx as i32;
    let mut stack : Vec<i32> = Vec::new();
    while object_dependency[i as usize] >= 0 {
        stack.push(i);
        i = object_dependency[i as usize];
    }

    return stack;
}

fn main() {
    let filename = "../part1/src/input";
    // let filename = "src/input0";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    const MAX_SIZE : usize = 2000;

    let mut object_dependency : Vec<i32> = vec![-1; MAX_SIZE];
    let mut object_index = HashMap::new();
    let mut idx : i32 = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        let oo_pair : Vec<&str> = line.split(")").collect();
        let origin = String::from(oo_pair[0]);
        let orbit = String::from(oo_pair[1]);

        let mut origin_idx = idx;        
        if !object_index.contains_key(&origin) {
            object_index.insert(origin, idx);
            idx += 1;            
        } else {
            origin_idx = *object_index.get(&origin).unwrap();
        }

        let mut orbit_idx = idx;
        if !object_index.contains_key(&orbit) {
            object_index.insert(orbit, idx);
            idx += 1;
        } else {
            orbit_idx = *object_index.get(&orbit).unwrap();
        }

        object_dependency[orbit_idx as usize] = origin_idx;
    }

    let you_idx = *object_index.get("YOU").unwrap();
    let san_idx = *object_index.get("SAN").unwrap();

    let you_path = get_origin_path(&object_dependency, you_idx as usize);
    let san_path = get_origin_path(&object_dependency, san_idx as usize);


    for i in 0..you_path.len() {
        let idx : i32 = match san_path.iter().position(|&r| r == you_path[i]) {
            Some(x) => x as i32,
            None => -1,
        };
        if idx != -1 {
            println!("Solution: {}", idx + (i as i32) - 2);
            break;
        }
    }

    // println!("you path: {:?}", you_path);
    // println!("san path: {:?}", san_path);
    // println!("object index: {:?}", object_index);
}
