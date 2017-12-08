const SIZE:usize = 1000;

fn main() {
    let mut array = [[0;SIZE];SIZE];
    let mut i = 1;
    let mut x = SIZE/2;
    let mut y = SIZE/2;

    let mut limit_x = 0;
    let mut limit_y = 0;

    array[y][x] = i;
    i += 1;

    loop {

        if i > 900000 {
            for (_,u) in array.iter().enumerate() {
                for (_,v) in u.iter().enumerate() {
                    print!("{:06} ", v)
                }
                println!()
            }

            break;
        }

        limit_x += 1;
        for j in 0..limit_x {
            x += 1;
            array[y][x] = i;
            i += 1;
            if find_value(265149, array[y][x], x, y) { break }
        }
        
        limit_y += 1;
        for j in 0..limit_y {
            y += 1;
            array[y][x] = i;
            i += 1;

            if find_value(265149, array[y][x], x, y) { break }
        }
        limit_x += 1;
        for j in 0..limit_x {
            x -= 1;
            array[y][x] = i;
            i += 1;

            if find_value(265149, array[y][x], x, y) { break }
        }
        limit_y += 1;
        for j in 0..limit_y {
            y -= 1;
            array[y][x] = i;
            i += 1;
            if find_value(265149, array[y][x], x, y) { break }
        }
    };
}

fn find_value(value: i32, position: i32, x: usize, y: usize) -> bool {
    let mut steps = 0;
    if value == position {
        if x > SIZE/2 {
            steps += x-SIZE/2;
        } else {
            steps += SIZE/2-x;
        }
        if y > SIZE/2 {
            steps += y-SIZE/2;
        } else {
            steps += SIZE/2-y;
        }

        println!("Steps: {}",steps);
        return true
    }
    false

}

