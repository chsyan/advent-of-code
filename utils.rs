use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn b_str_to_int(str:String) -> i32 {
    let chars: Vec<char> = str.chars().collect();
    let mut num = 0;
    for i in 0..chars.len() {
        if chars[i] == '1' {
            num = num | (1 << (chars.len() - i - 1));
        }
    }
    num
}

pub fn b_chars_int(chars:Vec<char>) -> i32 {
    let mut num = 0;
    for i in 0..chars.len() {
        if chars[i] == '1' {
            num = num | (1 << (chars.len() - i - 1));
        }
    }
    num
}

pub fn read_file_str(file:&str) -> Vec<String> {
    let mut data:Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(str) = line {
                data.push(str);
            }
        }
    }
    return data;
}

pub fn read_file_char(file:&str) -> Vec<Vec<char>> {
    let mut data:Vec<Vec<char>> = Vec::with_capacity(1000);
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(str) = line {
                data.push(str.chars().collect());
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

