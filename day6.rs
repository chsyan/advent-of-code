mod utils;

fn main() {
    let input = utils::read_file_str("./day6.txt");
    let list_str: Vec<&str> = input[0].split(",").collect();
    let mut vec: Vec<i64> = Vec::with_capacity(list_str.len());
    for i in 0..list_str.len() {
        vec.push(list_str[i].to_string().parse::<i64>().unwrap());
    }
    let list_num = vec.clone();
    for day in 0..80 {
        for i in 0..vec.len() {
            if vec[i] == 0 {
                vec[i] = 6;
                vec.push(8)
            } else {
                vec[i] -= 1;
            }
        }
    }
    println!("Part 1: {}", vec.len());


    vec = vec![0; 9];
    for i in 0..list_num.len() {
        vec[list_num[i] as usize] += 1;
    }
    for i in 0..256 {
        let mut aux: Vec<i64> = vec![0; 9];
        for i in 0..vec.len()-1 {
            aux[i] = vec[i+1];
        }
        aux[8] += vec[0];
        aux[6] += vec[0];
        vec = aux;
    }
    let mut count = 0;
    for i in 0..vec.len() {
        count += vec[i];
    }
    println!("Part 2: {}", count);
}
