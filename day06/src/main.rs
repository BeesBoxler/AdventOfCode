fn main() {
    let input = String::from("14	0	15	12	11	11	3	5	1	6	8	4	9	1	8	4");
    // let input = String::from("0	2	7	0");
    let mut input:Vec<i32> = input.trim().split('\t').map(|s| s.parse().ok().unwrap()).collect();
    println!("{:?}",input);
    let mut loop_size = 0;
    let mut sum = 0;
    let mut history:Vec<Vec<i32>> = Vec::new();
    let mut pos = 0;
    'loopy: loop{
        sum += 1;
        let mut max = (0,0);
        for (i,v) in input.clone().iter().enumerate() {
            if v > &max.1 { 
                max.1 = v.clone();
                max.0 = i;
            }
        }
        input[max.0] = 0;
        let mut i = max.0 + 1;
        for j in 0..max.1 {
            if i == input.len()   {
                i = 0;
            }
            input[i] += 1;
            i += 1;
        }

        // Part two 

        

        // Part one

        for (j,k) in history.iter().enumerate() {
            if &input[..] == &k[..] {
                println!("{:?} : {:?}", &input[..], &k[..]);
                pos = j+1;
                break 'loopy
            }
        }
        println!("{}: {:?}", sum, input);
        history.push(input.clone());
    }
    println!("{}", sum - pos)
}
