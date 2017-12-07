fn main() {
    const SIZE:usize = 50;
    let mut array = [[0;SIZE];SIZE];
    let mut i = 1;
    let mut x = SIZE/2;
    let mut y = SIZE/2;
    array[y][x] = i;
    for (_,u) in array.iter().enumerate() {
        for (_,v) in u.iter().enumerate() {
             print!("{} ", v)
        }
        println!()
    }
}
