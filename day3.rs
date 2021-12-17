mod utils;

fn main() {
    let lines = utils::read_file_char("./day3.txt");
    let line_size = lines[0].len();
    let mut count = vec![0; line_size];
    for line in &lines {
        for i in 0..line_size {
            match line[i] {
                '0' => count[i] -= 1,
                _   => count[i] += 1,
            }
        }
    }
    let mut gamma = 0;
    for i in 0..count.len() {
        if count[i] > 0 {
            gamma = gamma | (1 << (count.len() - i - 1));
        }
    }
    let epsilon = gamma ^ ((1 << count.len()) - 1); // Flip gamma bits
    println!("Part 1: {}", gamma * epsilon);

    let mut o2_lines = lines.clone();
    let mut co2_lines = lines.clone();
    for i in 0..count.len() {
        if o2_lines.len() > 1 {
            let most = most_b_digit(o2_lines.clone(), i);
        }
        
    }

    // let o2_rating  = utils::b_str_to_int(o2_lines[0].clone());
    // let co2_rating = utils::b_str_to_int(co2_lines[0].clone());
    // println!("Part 2: {:?}", o2_rating * co2_rating);
}

fn most_b_digit(lines:Vec<Vec<char>>, index:usize) -> i32 {
    let mut count = 0;
    for line in lines {
        if line[index] == '0' {
            count -= 1;
        } else {
            count += 1;
        }
    }
    if count > 1 {
        1
    } else if count < 1 {
        0
    } else {
        -1
    }
}
