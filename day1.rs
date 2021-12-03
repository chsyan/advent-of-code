mod utils;

fn main() {
    let lines = utils::read_file_as_num("./day1.txt");
    
    let mut count = 0;
    for i in 1..lines.len() {
        if lines[i] > lines[i-1] {
            count += 1;
        }
    }
    println!("Part 1: {}", count);

    count = 0;
    for i in 0..lines.len()-3 {
        if lines[i+3] > lines[i] {
            count += 1;
        }
    }
    println!("Part 2: {}", count);
}
