

/* 
 *
 *   ___        _     _   _    _      
 *  | __|  _ __| |__ | |_| |_ (_)___  
 *  | _| || / _| / / |  _| ' \| (_-<  
 *  |_|_\_,_\__|_\_\  \__|_||_|_/__/  
 *  |_ _|  __ _(_)_ _____   _  _ _ __ 
 *   | |  / _` | \ V / -_) | || | '_ \
 *  |___| \__, |_|\_/\___|  \_,_| .__/
 *        |___/                 |_|   
 *
*/













extern crate petgraph;

use std::io::prelude::*;
use std::fs::File;
use petgraph::Graph;
use petgraph::algo::*;
use petgraph::dot::{Dot, Config};
use petgraph::visit::*;


fn main() {
    let mut file = File::open("input.txt").expect("whoops");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("uh-oh");

    let mut graph = Graph::<i32,i32, petgraph::Undirected>::new_undirected();

    let input:Vec<Vec<&str>> = input.split('\n').map(|x| x.split(' ').collect()).collect();
    let mut inputs:Vec<Vec<i32>> = Vec::new();
    let mut sum = 0;
    for i in &input {
        let mut buffer:Vec<i32> = Vec::new();
        for (j,v) in i.iter().enumerate() {
            if j < 2 {
                continue
            }
            buffer.push(v.trim_right_matches(',').parse().ok().unwrap());
        }
        inputs.insert(i[0].parse().ok().unwrap(),buffer.clone());
    };

    let mut vec = Vec::new();


    for (i,_) in inputs.iter().enumerate() {
        let i = i as i32;
        vec.push(graph.add_node(i));
    }
    for (i,v) in inputs.iter().enumerate() {
        for j in v {
            let j = j.clone() as usize;
            graph.add_edge(vec[i],vec[j],0);
        }
    }
    // println!("{:?}",graph)
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    println!("{}", sum)
}
