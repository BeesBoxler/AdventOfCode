use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").expect("File not found.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Whoops something went wrong.");

    let input:Vec<&str> = input.trim().split('\n').collect();
    let input:Vec<Vec<&str>> = input.into_iter().map(|s| s.trim().split("->").map(|s| s.trim()).collect()).collect();

    println!("{:?}", input);
    
}
