use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn calculate_len(object_dependency_len : &mut Vec<i32>, object_dependency : &Vec<i32>, idx : usize) {
    let mut i = idx as i32;
    let mut stack : Vec<i32> = Vec::new();
    while object_dependency[i as usize] >= 0 {
        stack.push(i);
        i = object_dependency[i as usize];
    }

    let mut counter : i32 = 1;
    loop {
        let stack_idx = match stack.pop() {
            Some(x) => x,
            None => break,
        };

        object_dependency_len[stack_idx as usize] = counter;
        counter += 1;
    }
}

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    const MAX_SIZE : usize = 2000;

    let mut object_dependency : Vec<i32> = vec![-1; MAX_SIZE];
    let mut object_dependency_len : Vec<i32> = vec![0; MAX_SIZE];
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

    for i in 0..object_dependency_len.len() {
        if object_dependency_len[i] == 0 {
            calculate_len(&mut object_dependency_len, &object_dependency, i);
        }
    }

    let dependencies : i32 = object_dependency_len.iter().sum();

    println!("Solution: {}", dependencies);
    // println!("dependecies: {:?}", object_dependency);
    // println!("dependecies len: {:?}", object_dependency_len);
    // println!("object index: {:?}", object_index);
}
