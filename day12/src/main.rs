use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").expect("whoops");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("uh-oh");
    let input:Vec<Vec<&str>> = input.split('\n').map(|x| x.split(' ').collect()).collect();
    println!("{:?}",input);
    let mut inputs:HashMap<&str, Vec<&str>> = HashMap::new();
    let mut sum = 0;
    for i in input {
        let mut buffer:Vec<&str> = Vec::new();
        for (j,v) in i.iter().enumerate() {
            if j < 2 {
                continue
            }
            buffer.push(v.trim_right_matches(','));
        }
        inputs.insert(i[0],buffer);
        println!("{}: {:?}",i[0],inputs[i[0]])
    };
    for (i,v) in &inputs {
        // if i == &v[0] {
        //     sum += 1;
        // }
        if walk_tree(&inputs, i, i) {
            sum += 1;
        }
    }
    println!("Sum: {}", sum)
}

fn walk_tree(tree:&HashMap<&str,Vec<&str>>, name:&str, root:&str)->bool{
    for c in &tree[name] {
        if c == &"0" { return true }
        if c == &name || c == &root { return false }
        if walk_tree(&tree, c, &name) { return true };
        println!("{}",c);
        return false
    }
    false
}