use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
struct Tree {
    name: String,
    weight: i32,
    children: Vec<String>
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found.");
    let mut input = String::new();
    let mut root:String = String::new();
    file.read_to_string(&mut input).expect("Whoops something went wrong.");

    let input:Vec<&str> = input.trim().split('\n').collect();
    let mut trees: HashMap<String,Tree> = HashMap::new();

    for u in input {
        let buffer: Vec<&str> = u.trim().split(' ').collect();
        let name: String = String::from(buffer[0]);
        let weight:i32 = buffer[1].trim_left_matches('(').trim_right_matches(')').parse::<i32>().ok().unwrap();
        let mut children: Vec<String> = Vec::new();
        for (i, u) in buffer.iter().enumerate() {
            if i < 3 { continue }
            children.push(String::from(u.trim_right_matches(',')));
        };
        trees.insert(name.clone(), 
                     Tree{
                         name,
                         weight,
                         children
                        }
                    );
                
    }

    // for (i,v) in &trees {
    //     println!("{}: {}",i,get_weights(&trees,i));
    // }

    for (i,_) in &trees {
        let mut found = false;
        'find: for (_,v) in &trees {
            for k in &v.children {
                if &k == &i {
                    found = true;
                    break 'find
                }
            }
        }
        if !found {
            root = i.clone();
            println!("{}", i)
        }
    }    

    match_weights(&trees, &root);

}

fn match_weights(trees:&HashMap<String,Tree>, name: &str) {
    let tree = &trees[name];
    // println!("{:?}",tree);
    let weights:Vec<i32>= tree.children.iter().map(|c| get_weights(trees, c)).collect();
    for w in &weights {
        if w != &weights[0] {
            println!("Weight mismatch: {}: {}", name, w);
            break
        }
    }
    if tree.children.len() > 0 {
        println!(
            "{} ({}) -> {:?}",
            name,
            tree.weight,
            tree.children.iter().zip(weights).collect::<Vec<_>>()
        );
    } else {
        println!("{} ({})", name, tree.weight);
    }
    for c in &tree.children {
        match_weights(&trees, c);
    }
}

fn get_weights(trees:&HashMap<String,Tree>, name: &str) -> i32{
    let mut weight = 0;

    let tree = &trees[name];
    weight += tree.weight;

    for child in &tree.children {
        weight += get_weights(trees, child);
    }

    weight
}
