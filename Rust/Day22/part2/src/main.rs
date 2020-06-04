extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::{Zero, One};
use num_traits::ToPrimitive;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, PartialEq, Debug)]
enum DealType {
    NewStack,
    WithIncrement(i64),
    Cut(i64),
}

fn reorder(input : Vec<DealType>, count : i64) -> Vec<DealType> {
    // Compact "deal into stack" shuffles.
	//
	// Two consecutive "deal into stack" shuffles cancel each other. So we
	// iterate over the input list, tracking whether we need to currently need
	// reverse the stack, which changes every time we see a "deal into stack"
	// shuffle. Then, if we need to reverse at the end, we add a single "deal
	// into stack" shuffle to the output.
	//
	// If we currently need to reverse the stack, we have to modify the other
	// shuffles. This boils down to the following two rules, where the list of
	// instructions below the line has the same effect as the list of
	// instructions above the line:
	//
	// deal into new stack
	// cut x
	// -------------------
	// cut count-x
	// deal into new stack
	//
	// deal into new stack
	// deal with increment x
	// ---
	// deal with increment x
	// cut count+1-x
	// deal into new stack
	//
    let mut compacted = Vec::new();
    compacted.reserve(input.len());
    let mut reverse = false;
    for shuffle in input.iter() {
        if *shuffle == DealType::NewStack {
            reverse = !reverse;
            continue;
        }
        if !reverse {
            compacted.push(*shuffle);
            continue;
        }
        match shuffle {
            DealType::WithIncrement(incr) => {
                compacted.push(*shuffle);
                compacted.push(DealType::Cut(count + 1 - incr));
            },
            DealType::Cut(n) => {
                let mut cut = (n + count) % count; // normalize negative values
                cut = count - cut;                 // reverse cut
                compacted.push(DealType::Cut(cut));
            },
            DealType::NewStack => unreachable!()
        };
    }
    if reverse {
        compacted.push(DealType::NewStack);
    }

    return compacted;
}

fn reorder2(input : Vec<DealType>, count : i64) -> Vec<DealType> {
    // Compact "cut" shuffles.
	//
	// Here we require that the "deal into stack" shuffles have been compacted
	// already, so we can insert the "cut" shuffle before the "deal into
	// stack" shuffle or at the end. Then, we only have to handle "deal with
	// increment" shuffles.
	//
	// cut x
	// cut y
	// ---
	// cut (x+y) % count
	//
	// cut x
	// deal with increment y
	// ---
	// deal with increment y
	// cut (x*y) % count
	//
    let mut compacted = Vec::new();
    compacted.reserve(input.len());
    let mut cut: BigInt = Zero::zero();
    for shuffle in input.iter() {
        match shuffle {
            DealType::NewStack => {
                if cut != Zero::zero() {
                    compacted.push(DealType::Cut(cut.to_i64().unwrap()));
                    cut = Zero::zero();
                }
                compacted.push(*shuffle);
            },
            DealType::WithIncrement(incr) => {
                let bi_incr = incr.to_bigint().unwrap();
                compacted.push(*shuffle);
                cut = (cut * bi_incr) % count;
            },
            DealType::Cut(n) => {
                let bi_n = n.to_bigint().unwrap();
                cut = (cut + bi_n) % count;
            }
        };
    }
    if cut != Zero::zero() {
        compacted.push(DealType::Cut(cut.to_i64().unwrap()));
    }
    return compacted;
}

fn reduce(input : Vec<DealType>, count : i64) -> Vec<DealType> {
    // Compact "deal with increment" shuffles.
	//
	// Finally, we just have to combine "deal with increment" shuffles.
	//
	// deal with increment x
	// deal with increment y
	// ---
	// deal with increment (x*y) % count
	//
    let mut compacted = Vec::new();
    compacted.reserve(input.len());
    let mut increment : BigInt = One::one();
    for shuffle in input.iter() {
        match shuffle {
            DealType::WithIncrement(incr) => {
                let bi_incr = incr.to_bigint().unwrap();
                increment = (increment * bi_incr) % count;
            },
            _ => {
                if increment != One::one() {
                    compacted.push(DealType::WithIncrement(increment.to_i64().unwrap()));
                    increment = One::one();
                }
                compacted.push(*shuffle);
            }
        }
    }
    if increment != One::one() {
        compacted.push(DealType::WithIncrement(increment.to_i64().unwrap()));
    }
    return compacted;
}

fn compact(input : Vec<DealType>, count : i64) -> Vec<DealType> {
    let reorder_input = reorder(input, count);
    let reorder2_input = reorder2(reorder_input, count);
    return reduce(reorder2_input, count);
}

fn main() {
    let filename = "../part1/src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut shuffle = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.trim().len() == 0 {
            break;
        }

        if line.starts_with("deal w") {
            let incr: i64 = line[20..].parse().unwrap();
            shuffle.push(DealType::WithIncrement(incr));
        } else if line.starts_with("cut") {
            let n: i64 = line[4..].parse().unwrap();
            shuffle.push(DealType::Cut(n));
        } else if line.starts_with("deal i") {
            shuffle.push(DealType::NewStack);
        } else {
            unreachable!();
        }
    }

    const COUNT : i64 = 119315717514047;
    const ITERATIONS : i64 = 101741582076661;

    let mut shuffles = Vec::new();

    // Exponentiation by squaring.
    let mut iterations_left = COUNT - ITERATIONS - 1;
    while iterations_left != 0 {
        if iterations_left % 2 == 1 {
            shuffles.append(&mut shuffle.clone());
            shuffles = compact(shuffles, COUNT);
        }
        
        shuffle.append(&mut shuffle.clone());
        shuffle = compact(shuffle, COUNT);

        iterations_left /= 2;
    }

    let mut pos : i64 = 2020;
    for shuffle in shuffles.iter() {
        match shuffle {
            DealType::WithIncrement(incr) => {
                let increment = incr;
                pos = (pos * increment) % COUNT;
            },
            DealType::NewStack => {
                pos = COUNT - 1 - pos;
            },
            DealType::Cut(n) => {
                if pos < *n {
                    pos = pos + COUNT - n;
                } else {
                    pos = pos - n;
                }
            }
        };
    }

    println!("Card in pos: {}", pos);
}
