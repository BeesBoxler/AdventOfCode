fn main() {
    // Setup the lengths and arrays and what-not
    let mut list = vec![0;256];
    let lengths = String::from("70,66,255,2,48,0,54,48,80,141,244,254,160,108,1,41");

    // TEST
    // let mut list = vec![0;5];
    // let lengths = String::from("3,4,1,5");

    let lengths:Vec<i32> = lengths.split(',').map(|x| x.parse().ok().unwrap()).collect();

    
    let mut skip = 0;
    let mut pos = 0;
    for i in 0..256 {
        list[i as usize] = i;
    }

    // Now for the meat 🥩
    println!("{:?}", list);

    for i in lengths {
        for j in 0..i/2 {
            let x = (((pos+j)%256)) as usize;
            let y = (((i-j-1+pos)%256)) as usize;
            println!("x: {}, y: {}",x,y);
            list.swap(x,y);
        }
        println!("{:?}",list);
        pos += skip + i;
        skip += 1;

    }
    println!("{:?}",list);

    println!("{}", list[0]*list[1])


}
