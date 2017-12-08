use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "passwords.txt";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mut password_array: Vec<Vec<&str>> = Vec::new();
    {
        let passwords: Vec<&str> = contents.trim().split('\n').collect();

        for (_,p) in passwords.iter().enumerate() {
            let password = p.trim().split(' ').collect();
            password_array.push(password);
        }
    }
    let mut sum = password_array.len();
    for (_,p) in password_array.iter().enumerate() {
        'password_loop: for (j,q) in p.iter().enumerate() {
            for (k,r) in p.iter().enumerate() {
                if r == q && j != k{
                    sum -= 1;
                    break 'password_loop
                }
            }
        }
    }
    println!("{}", sum);

    let mut sum = password_array.len();
    for (_,p) in password_array.iter().enumerate() {
        'anagram_loop: for (j,q) in p.iter().enumerate() {
            for (k,r) in p.iter().enumerate() {
                if word_fingerprint(q) == word_fingerprint(r) && j != k{
                    sum -= 1;
                    break 'anagram_loop
                }
            }
        }
    }
    println!("{}", sum);

}

fn word_fingerprint(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.iter().collect::<String>()
}
