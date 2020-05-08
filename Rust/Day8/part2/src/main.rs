use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

// typedef struct {
//     char  idlength;
//     char  colourmaptype;
//     char  datatypecode;              // 2
//     short int colourmaporigin;
//     short int colourmaplength;
//     char  colourmapdepth;
//     short int x_origin;
//     short int y_origin;
//     short width;
//     short height;
//     char  bitsperpixel;
//     char  imagedescriptor;
//  } HEADER;

fn save_tgc(image : &[[u8; 25]; 6], height : usize, width : usize) -> std::io::Result<()> {
    let mut file = File::create("message.tga")?;

    let tga_header = [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, (width & 0x00FF) as u8, ((width & 0xFF00) / 256) as u8, (height & 0x00FF) as u8, ((height & 0xFF00) / 256) as u8, 24, 0];
    file.write_all(&tga_header)?;
    for r in (0..height).rev() {
        for c in 0..width {
            let color : u8 = if image[r][c] == 0 { 0 } else { 0xff };
            file.write_all(&[color, color, color])?;
        }
    }
    Ok(())
}

fn main() {
    // let filename = "src/input0";
    let filename = "../part1/src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut pixels_info : Vec<u8> = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        pixels_info = line.as_bytes().to_vec();
        break;
    }

    let pi : Vec<u8> = pixels_info.iter().map(|&x| x - ('0' as u8)).collect();

    // println!("{:?}", pi);

    const ROW : usize = 6;
    const COL : usize = 25;
    // const ROW : usize = 2;
    // const COL : usize = 2;
    const LAYER_SIZE : usize = ROW * COL;

    let mut final_image = [[10u8; COL]; ROW];
    for j in 0..LAYER_SIZE {
        let mut actual_pixel = 2u8;
        for i in 0..(pi.len() / LAYER_SIZE) {
            let pixel = pi[i * LAYER_SIZE + j];
            if pixel == 0 || pixel == 1 {
                actual_pixel = pixel;
                break;
            }
        }
        final_image[j/COL][j%COL] = actual_pixel;
    }

    for r in 0..ROW {
        for c in 0..COL {
            print!("{}", final_image[r][c]);
        }
        println!();
    }
    println!();
    println!();

    for r in 0..ROW {
        for c in 0..COL {
            print!("{}", if final_image[r][c] == 0 { ' ' } else { '0' });
        }
        println!();
    }

    save_tgc(&final_image, ROW as usize, COL as usize);
}
