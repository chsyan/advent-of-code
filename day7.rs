mod utils;

fn main() {
    let input = utils::read_file_str("./day7.txt");
    let list_str: Vec<&str> = input[0].split(",").collect();
    let mut vec: Vec<i64> = Vec::with_capacity(list_str.len());
    for i in 0..list_str.len() {
        vec.push(list_str[i].to_string().parse::<i64>().unwrap());
    }

    let mut max = 0;
    for i in 0..vec.len() {
        if vec[i] > max {
            max = vec[i];
        }
    }

    let mut min_fuel: i64 = i64::MAX;
    for i in 0..max {
        let mut fuel = 0;
        for j in 0..vec.len() {
            let diff: i64 = vec[j as usize] - i;
            fuel += diff.abs();
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    println!("Part 1: {}", min_fuel);


    min_fuel = i64::MAX;
    for i in 0..max {
        let mut fuel = 0;
        for j in 0..vec.len() {
            let diff: i64 = vec[j as usize] - i;
            let n = diff.abs();
            let cost = n * (n + 1) / 2;
            fuel += cost;
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    println!("Part 2: {}", min_fuel);

}

