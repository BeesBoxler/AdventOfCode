fn main() {
    let mut array:[[&str;128];128] = [[".";128];128];
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("pls input a string, yeah");
    let input = String::from("vbqugkhl");
    let sum = 0;
    for i in 0..127 {

        // TODO: Knot tying algorithm

        
        let mut line = String::from(format!("{}-{}",input,i));
        line = line.bytes().map(|b| format!("{:x}",b)).collect();
        let mut bits:String = String::new();


        for (j,b) in line.chars().enumerate() {
            // println!("{}",b);
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
                '1' => "#",

                _ => panic!()
            };
            print!("{}",array[i][j])

        }
        println!();
        // println!("{}",bits)
    }
    // println!("{:?}",array)
}
