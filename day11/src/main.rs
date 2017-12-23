use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Hero {
    x: i32,
    y: i32,
    z: i32
}

fn main() {

    let mut file = File::open("input.txt").expect("woah there");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("That boy ain't right"); //https://youtu.be/y1C8C7op9LU

    // let input = String::from("se,sw,se,sw,sw");

    let mut our_hero = Hero{x:0, y:0, z:0};
    let zero_hero = Hero{x:0, y:0, z:0};

    let input:Vec<&str> = input.split(",").collect();

    let mut max = 0;

    for i in input {
        match i {
            "n" => {
                our_hero.x += 0;
                our_hero.y += 1;
                our_hero.z -= 1;
            },
            "s" => {
                our_hero.x += 0;
                our_hero.y -= 1;
                our_hero.z += 1;
            },
            "ne" => {
                our_hero.x += 1;
                our_hero.y += 0;
                our_hero.z -= 1;
            },
            "sw" => {
                our_hero.x -= 1;
                our_hero.y += 0;
                our_hero.z += 1;
            },
            "nw" => {
                our_hero.x -= 1;
                our_hero.y += 1;
                our_hero.z += 0;
            },
            "se" => {
                our_hero.x += 1;
                our_hero.y -= 1;
                our_hero.z -= 0;
            },

            _ => {}
        }

        if hex_distance(&zero_hero,&our_hero) > max {
            max = hex_distance(&zero_hero,&our_hero)
        }
    };

    println!("Distance from start: {}", hex_distance(&zero_hero,&our_hero));
    println!("Maximum distance from start: {}", max);

}

fn hex_distance(a:&Hero, b:&Hero) -> i32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) / 2
}