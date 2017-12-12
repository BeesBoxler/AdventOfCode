use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
    let mut file = File::open("instructions.txt").expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Something bad happened.");

    let input:Vec<&str> = input.trim().split('\n').collect();
    let mut instruction_groups:Vec<Vec<&str>> = Vec::new();
    let mut registers:HashMap<&str, i32> = HashMap::new();
    for i in input {
        let instruction:Vec<&str>= i.trim().split(' ').collect();
        instruction_groups.push(instruction.clone());
        registers.insert(instruction.get(0).unwrap(),0);
    }
    let mut absolute_max = 0;
    for i in instruction_groups {
        match i[1] {
            "inc" => {
                match i[5] {
                    ">" => { 
                        if &registers[i[4]] > &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register += i[2].parse::<i32>().unwrap();
                        } 
                    },
                    "<" => { 
                        if &registers[i[4]] < &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register += i[2].parse::<i32>().unwrap();
                        } 
                    },
                    ">=" => { 
                        if &registers[i[4]] >= &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register += i[2].parse::<i32>().unwrap();
                        } 
                    },
                    "<=" => { 
                        if &registers[i[4]] <= &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register += i[2].parse::<i32>().unwrap();
                        } 
                    },
                    "==" => { 
                        if &registers[i[4]] == &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register += i[2].parse::<i32>().unwrap();
                        } 
                    },
                    "!=" => { 
                        if &registers[i[4]] != &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register += i[2].parse::<i32>().unwrap();
                        } 
                    },
                    _ =>  {
                        panic!("Unknown instruction")
                    }
                }
            },
            "dec" => {
                match i[5] {
                    ">" => { 
                        if &registers[i[4]] > &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register -= i[2].parse::<i32>().unwrap();
                        } 
                    },
                    "<" => { 
                        if &registers[i[4]] < &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register -= i[2].parse::<i32>().unwrap();
                        } 
                    },
                    ">=" => { 
                        if &registers[i[4]] >= &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register -= i[2].parse::<i32>().unwrap();
                        } 
                    },
                    "<=" => { 
                        if &registers[i[4]] <= &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register -= i[2].parse::<i32>().unwrap();
                        } 
                    },
                    "==" => { 
                        if &registers[i[4]] == &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register -= i[2].parse::<i32>().unwrap();
                        } 
                    },
                    "!=" => { 
                        if &registers[i[4]] != &i[6].parse::<i32>().unwrap() {
                            let register = registers.entry(i[0]).or_insert(0);
                            *register -= i[2].parse::<i32>().unwrap();
                        } 
                    },
                    _ =>  {
                        panic!("Unknown instruction")
                    }
                }
            },
            _ => { panic!("Unknown instruction") }
        }
        for (_,v) in &registers {
            if v > &absolute_max {
                absolute_max = v.clone();
            }

        }
    }
    let mut max:(&str,i32) = ("",0);
    for (i,v) in &registers {
        if v > &max.1 {
            max.1 = v.clone();
            max.0 = i;
        }
    }
    println!("Highest Value is {:?}", max);
    println!("Highest at any point is {}", absolute_max);
    
    // println!("{:?}",registers);
}