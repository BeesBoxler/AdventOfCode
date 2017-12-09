fn main() {
    let input = String::from("14	0	15	12	11	11	3	5	1	6	8	4	9	1	8	4");
    let input:Vec<i32> = input.trim().split('\t').map(|s| s.parse().ok().unwrap()).collect();
    println!("{:?}", input)
}
