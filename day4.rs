mod utils;

fn main() {
    // Load data.
    let lines = utils::read_file_str("./day4.txt");
    let nums_str: Vec<&str> = lines[0].split(",").collect();
    let mut nums: Vec<i32> = Vec::with_capacity(nums_str.len());
    for str in &nums_str {
        nums.push(str.parse::<i32>().unwrap());
    }
    let nums_original = nums.clone();

    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut i = 2;
    while i < lines.len() {
        let mut board: Vec<Vec<i32>> = Vec::new();
        while i < lines.len() && !lines[i].is_empty() {
            let data_str: Vec<&str> = lines[i].split_whitespace().collect();
            let mut data_num: Vec<i32> = Vec::new();
            for str in &data_str {
                data_num.push(str.parse::<i32>().unwrap());
            }
            i += 1;
            board.push(data_num);
        }
        boards.push(board);
        i += 1;
    }
    let boards_original = boards.clone();

    'part1:for num in nums {
        let mut bingo = false;
        for board in boards.iter_mut() {
            for row in board {
                for n in row.iter_mut() {
                    if *n == num {
                        *n = -1;
                    }
                }
            }
        }

        for board in boards.iter() {
            'row:for row in board {
                for i in 0..row.len() {
                    if row[i] != -1 {
                        break;
                    }
                    if i == row.len() - 1 {
                        bingo = true;
                        break 'row;
                    }
                }
            }
            'col:for col in 0..board[0].len() {
                for row in 0..board.len() {
                    if board[row][col] != -1 {
                        break;
                    }
                    if row == board.len() {
                        bingo = true;
                        break 'col;
                    }
                }
            }
            if bingo {
                let mut ans = 0;
                for row in board {
                    for val in row {
                        if *val != -1 {
                            ans += *val;
                        }
                    }
                }
                println!("Part 1: {}", ans * num);
                break 'part1;
            }
        }
    }
    
    boards = boards_original;
    nums = nums_original;
    'part2:for num in nums {
        for board in boards.iter_mut() {
            for row in board {
                for n in row.iter_mut() {
                    if *n == num {
                        *n = -num;
                        if num == 0 {
                            *n = -100;
                        }
                    }
                }
            }
        }
        let mut idx:i32 = boards.len() as i32 - 1;
        while idx >= 0 {
            let mut bingo = false;
            let index = idx as usize;
            'row1:for row in 0..boards[index].len() {
                for i in 0..boards[index][row].len() {
                    if boards[index][row][i] > - 1 {
                        break;
                    }
                    if i == boards[index][row].len() - 1 {
                        bingo = true;
                        break 'row1;
                    }
                }
            }
            'col1:for col in 0..boards[index][0].len() {
                for row in 0..boards[index].len() {
                    if boards[index][row][col] > -1 {
                        break;
                    }
                    if row == boards[index].len() - 1 {
                        bingo = true;
                        break 'col1;
                    }
                }
            }
            if bingo {
                let mut ans = 0;
                for row in 0..boards[index].len() {
                    for val in 0..boards[index][row].len() {
                        if boards[index][row][val] >= 0 {
                            ans += boards[index][row][val];
                        }
                    }
                }
                if boards.len() == 1 {
                    println!("Part 2: {}", ans * num);
                    break 'part2;
                }
                boards.remove(index);
            }
            idx -= 1;
        }
    }
}
