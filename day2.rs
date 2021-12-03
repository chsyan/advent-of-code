mod utils;

fn main() {
    let lines = utils::read_file("./day2.txt");
    let mut depth = 0;
    let mut depth2 = 0;
    let mut pos = 0;
    let mut aim = 0;
    for line in lines {
        let data = line.split(' ').collect::<Vec<&str>>();
        let val = data[1].parse::<i32>().unwrap();
        match data[0] {
            "forward" => {
                pos += val;
                depth2 += aim * val;
                }
            "down" => {
                depth += val;
                aim += val;
            },
            _ => { // "up"
                depth -= val;
                aim -= val;
            },
        }
    }
    println!("Part 1: {}", depth * pos);
    println!("Part 2: {}", depth2 * pos);
}
