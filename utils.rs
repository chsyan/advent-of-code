use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(file:&str) -> Vec<String> {
    let mut data:Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(ip) = line {
                data.push(ip);
            }
        }
    }
    return data;
}

pub fn read_file_as_num(file:&str) -> Vec<i32> {
    let mut data:Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(ip) = line {
                data.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    return data;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

