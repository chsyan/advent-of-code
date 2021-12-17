mod utils;

fn main() {
    let input = utils::read_file_str("./day8test.txt");
    let mut lines: Vec<Vec<Vec<Vec<char>>>> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let in_out_split: Vec<&str> = input[i].split(" | ").collect::<Vec<&str>>();
        let mut line: Vec<Vec<Vec<char>>> = Vec::new();
        for j in 0..2 {
            let mut inner: Vec<Vec<char>> = Vec::new();
            let part = in_out_split[j].split(" ").collect::<Vec<&str>>();
            for k in 0..part.len() {
                let mut chars = part[k].chars().collect::<Vec<char>>();
                chars.sort();
                inner.push(chars);
            }
            line.push(inner);
        }
        lines.push(line);
    }

    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i][1].len() {
            match lines[i][1][j].len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {},
            }
        }
    }
    println!("Part 1: {}", count);


    //  1111
    // 2    3
    // 2    3
    //  4444
    // 5    6
    // 5    6
    //  7777
    //  aaaa 
    // b    c
    // b    c
    //  dddd 
    // e    f
    // e    f
    //  gggg 
    let mut sum = 0;
    for i in 0..lines.len() {
        // find 1,4,7,8
        for j in 0..lines[i][0].len() {
            let mut seq: Vec<Vec<char>> = vec![vec![]; 10];
            match lines[i][0][j].len() {
                2 => seq[1] = lines[i][0][j].clone(),
                4 => seq[4] = lines[i][0][j].clone(),
                3 => seq[7] = lines[i][0][j].clone(),
                8 => seq[8] = lines[i][0][j].clone(),
                _ => {},
            }
            println!("{:?}", seq);
        }
    }
    println!("Part 2: {}", sum);
}
