use std::fs::File;
use std::io::{BufRead, BufReader};

fn deal_new_stack(original : &mut Vec<usize>) {
    original.reverse();
}

fn cut_n_cards(original : &mut Vec<usize>, n : i32) {
    let c = original.clone();
    if n > 0 {
        let nn = n as usize;
        let bi = original.len() - nn;
        original[bi..].copy_from_slice(&c[0..nn]);
        original[0..bi].copy_from_slice(&c[nn..]);
    } else {
        let nn = -n as usize;
        let bi = original.len() - nn;
        original[0..nn].copy_from_slice(&c[bi..]);
        original[nn..].copy_from_slice(&c[0..bi]);
    }
}

fn deal_with_increment(original : &mut Vec<usize>, i : usize) {
    let c = original.clone();
    let mut pos = 0;
    for j in 0..c.len() {
        original[pos] = c[j];
        pos = (pos + i) % c.len();
    }
}

fn main() {
    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut shufle = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.trim().len() == 0 {
            break;
        }

        shufle.push(line);
    }

    // const MAX_CARD_COUNT : usize = 10;
    const MAX_CARD_COUNT : usize = 10007;

    let mut cards = Vec::new();
    for i in 0..MAX_CARD_COUNT {
        cards.push(i);
    }

    for l in shufle.iter() {
        if l.starts_with("deal w") {
            let incr : usize = l[20..].parse().unwrap();
            // println!("{}", incr);
            deal_with_increment(&mut cards, incr);
        } else if l.starts_with("cut") {
            let n : i32 = l[4..].parse().unwrap();
            // println!("{}", n);
            cut_n_cards(&mut cards, n);
        } else if l.starts_with("deal i") {
            deal_new_stack(&mut cards);
        } else {
            unreachable!();
        }
    }

    // println!("{:?}", cards);
    let i = cards.iter().position(|&a| a == 2019).unwrap();
    println!("Card 2019 is in pos: {}", i);
}
