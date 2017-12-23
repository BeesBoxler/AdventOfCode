use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("woah, oops");

    let mut sum = 0;
    let mut level = 0;
    let mut chars = input.chars();
    let mut garbage = false;
    let mut trash = 0;

    while let Some(c) = chars.next() {
        match c {
            '!' => {chars.next();},
            '>' => garbage = false,
            _ if garbage => trash += 1,
            '<' => garbage = true,
            '{' => level += 1,
            '}' => { sum += level; level -= 1;}
            _ => {}
        }
    }

    println!("Chunks: {}",sum);
    println!("Garbage chrs: {}",trash);
}
