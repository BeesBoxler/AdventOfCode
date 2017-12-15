fn main() {
    // Setup the lengths and arrays and what-not
    let lengths = String::from("70,66,255,2,48,0,54,48,80,141,244,254,160,108,1,41");
    println!("{}",generate_knot_hash(&lengths));
    // let lengths = String::from("AoC 2017");
}

fn generate_knot_hash(lengths:&str) -> String {

    let mut list = vec![0;256];
    let mut n_lengths:Vec<i32> = Vec::new();

    for i in lengths.bytes() {
        n_lengths.push(i as i32);
    }
    n_lengths.extend(vec![17, 31, 73, 47, 23]);

    
    let mut skip = 0;
    let mut pos = 0;
    for i in 0..255 {
        list[i as usize] = i;
    }
    list[255] = 255;

    // Now for the meat 🥩
    // println!("{:?}", list);
    for _ in 0..64 {
        for i in  &n_lengths {
            for j in 0..i/2 {
                let x = (((pos+j)%256)) as usize;
                let y = (((i-j-1+pos)%256)) as usize;
                // println!("x: {}, y: {}",x,y);
                list.swap(x,y);
            }
            // println!("{:?}",list);
            pos += skip + i;
            skip += 1;

        }
    }
    // println!("{:?}",list);
    let mut dense_hash: Vec<u8> = vec![0;16];
    for i in 0..16 {
        for j in 0..16 {
            let j = j + i * 16;
            dense_hash[i] = dense_hash[i] ^ list[j];
            // println!("{}",dense_hash[i]);
        } 
    }
    // println!("{:?}", dense_hash);
    let dense_hash:String = dense_hash.iter().map(|x| format!("{:02x}",x)).collect();
    // println!("{}", dense_hash);
    dense_hash

}
