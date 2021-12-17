mod utils;
use std::collections::HashMap;
use std::cmp::{max, min};

fn main() {
    let input = utils::read_file_str("./day5.txt");
    let mut lines: Vec<Vec<i32>> = Vec::new();
    for i in 0..input.len() {
        let coords: Vec<&str> = input[i].split(" -> ").collect();
        let mut line: Vec<i32> = Vec::new();
        for j in 0..coords.len() {
            let points: Vec<&str> = coords[j].split(",").collect();
            line.push(points[0].parse::<i32>().unwrap());
            line.push(points[1].parse::<i32>().unwrap());
        }
        lines.push(line);
    }
    let mut map: HashMap<Vec<i32>, i32> = HashMap::with_capacity(input.len().pow(2));
    let mut count = 0;

    for i in 0..lines.len() {
        let x1 = lines[i][0];
        let y1 = lines[i][1];
        let x2 = lines[i][2];
        let y2 = lines[i][3];
        let mut x = min(x1, x2);
        let mut y = min(y1, y2);
        if y2 == y1 {
            while x <= max(x1, x2) {
                let coord = vec![x, y1];
                let val = match map.get(&coord) {
                    Some(v) => *v + 1,
                    _ => 1,
                };
                if val == 2 {
                    count += 1;
                }
                map.insert(coord, val);
                x += 1;
            }
        } else if x2 == x1 {
            while y <= max(y1, y2) {
                let coord = vec![x1, y];
                let val = match map.get(&coord) {
                    Some(v) => *v + 1,
                    _ => 1,
                };
                if val == 2 {
                    count += 1;
                }
                map.insert(coord, val);
                y += 1;
            }
        }
    }
    println!("Part 1: {}", count);
    
    // -----------------------------------------------------------------------------------------------------------------

    map = HashMap::with_capacity(input.len().pow(2));
    count = 0;

    for i in 0..lines.len() {
        let x1 = lines[i][0];
        let y1 = lines[i][1];
        let x2 = lines[i][2];
        let y2 = lines[i][3];
        let mut x = min(x1, x2);
        let mut y = min(y1, y2);
        if y2 == y1 {
            while x <= max(x1, x2) {
                let coord = vec![x, y1];
                let val = match map.get(&coord) {
                    Some(v) => *v + 1,
                    _ => 1,
                };
                if val == 2 {
                    count += 1;
                }
                map.insert(coord, val);
                x += 1;
            }
        } else if x2 == x1 {
            while y <= max(y1, y2) {
                let coord = vec![x1, y];
                let val = match map.get(&coord) {
                    Some(v) => *v + 1,
                    _ => 1,
                };
                if val == 2 {
                    count += 1;
                }
                map.insert(coord, val);
                y += 1;
            }
        } else {
            x = x1;
            y = y1;
            let x_inc = if x2 > x1 { 1 } else { -1 };
            let y_inc = if y2 > y1 { 1 } else { -1 };
            loop {
                let coord = vec![x, y];
                let val = match map.get(&coord) {
                    Some(v) => *v + 1,
                    _ => 1,
                };
                if val == 2 {
                    count += 1;
                }
                map.insert(coord, val);
                if y == y2 { break; }
                x += x_inc;
                y += y_inc;
            }
        }
    }
    println!("Part 2: {}", count);
}
