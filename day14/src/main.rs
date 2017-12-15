fn main() {
    let mut array:[[&str;128];128] = [[".";128];128];
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("pls input a string, yeah");
    let input = String::from("vbqugkhl");
    // let input = String::from("flqrgnkx");
    let mut sum = 0;
    for i in 0..128 {
        
        let mut line = String::from(format!("{}-{}",input,i));
        line = generate_knot_hash(&line);
        let mut bits:String = String::new();


        for (j,b) in line.chars().enumerate() {
            bits += match b {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'a' => "1010",
                'b' => "1011",
                'c' => "1100",
                'd' => "1101",
                'e' => "1110",
                'f' => "1111",

                _ => panic!()
            };
        }
        for (j,b) in bits.chars().enumerate() {
            array[i][j] = match b {
                '0' => ".",
                '1' => {
                        sum +=1;
                        "#"
                        },

                _ => panic!()
            };
            print!("{}",array[i][j])

        }
        println!();
        // println!("{}",bits)
    }
    println!("{}",sum)
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
    for _ in 0..64 {
        for i in  &n_lengths {
            for j in 0..i/2 {
                let x = (((pos+j)%256)) as usize;
                let y = (((i-j-1+pos)%256)) as usize;
                list.swap(x,y);
            }
            pos += skip + i;
            skip += 1;
        }
    }
    let mut dense_hash: Vec<u8> = vec![0;16];
    for i in 0..16 {
        for j in 0..16 {
            let j = j + i * 16;
            dense_hash[i] = dense_hash[i] ^ list[j];
        } 
    }
    let dense_hash:String = dense_hash.iter().map(|x| format!("{:02x}",x)).collect();
    dense_hash
}