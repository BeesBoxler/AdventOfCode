use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Tree {
    name: String,
    weight: i32,
    children: Vec<String>
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Whoops something went wrong.");

    let input:Vec<&str> = input.trim().split('\n').collect();
    let mut trees: Vec<Tree> = Vec::new();

    for u in input {
        let buffer: Vec<&str> = u.trim().split(' ').collect();
        let name: String = String::from(buffer[0]);
        let weight:i32 = buffer[1].trim_left_matches('(').trim_right_matches(')').parse::<i32>().ok().unwrap();
        let mut children: Vec<String> = Vec::new();
        for (i, u) in buffer.iter().enumerate() {
            if i < 3 { continue }
            children.push(String::from(u.trim_right_matches(',')));
        }
        trees.push(Tree{name,weight,children})
    }

    for i in &trees {
        let mut found = false;
        'find: for j in &trees {
            for k in &j.children {
                if k == &i.name {
                    found = true;
                    break 'find
                }
            }
        }
        if !found {
            println!("{}", i.name)
        }
    }    
}
