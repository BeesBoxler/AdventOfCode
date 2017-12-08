use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "jumps.txt";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mut jump_array: Vec<i32> = contents.trim().split('\n').map(|s| s.parse().unwrap()).collect();
    let mut i:i32 = 0;
    let mut sum = 0;
    loop {
        i += jump_array[i as usize];
        jump_array[i as usize] += 1;
        sum += 1;
        println!("{}",sum);
        if i > jump_array.len() as i32 || i < 0 { break }
        
    }

}
