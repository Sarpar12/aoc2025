use std::fs;

pub fn part1(input: &str) -> Result<i32, std::io::Error> {
    let contents = fs::read_to_string(input)?;
    let vals: Vec<Vec<char>> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let rows = vals.len();
    let cols = if rows > 0 { vals[0].len() } else { 0 };

    let mut num_arr: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

    // For each '@' in vals, add 1 to adjacent cells in num_arr
    for row in 0..rows {
        for col in 0..cols {
            if vals[row][col] == '@' {
                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let new_row = row as i32 + dr;
                        let new_col = col as i32 + dc;

                        if new_row >= 0
                            && new_row < rows as i32
                            && new_col >= 0
                            && new_col < cols as i32
                        {
                            num_arr[new_row as usize][new_col as usize] += 1;
                        }
                    }
                }
            }
        }
    }

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if vals[row][col] == '@' && num_arr[row][col] < 4 {
                count += 1;
            }
        }
    }

    Ok(count as i32)
}

pub fn part2(input: &str) -> Result<i32, std::io::Error> {
    let contents = fs::read_to_string(input)?;
    let mut vals: Vec<Vec<char>> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let rows = vals.len();
    let cols = if rows > 0 { vals[0].len() } else { 0 };

    let mut num_arr: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

    // For each '@' in vals, add 1 to adjacent cells in num_arr
    for row in 0..rows {
        for col in 0..cols {
            if vals[row][col] == '@' {
                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let new_row = row as i32 + dr;
                        let new_col = col as i32 + dc;

                        if new_row >= 0
                            && new_row < rows as i32
                            && new_col >= 0
                            && new_col < cols as i32
                        {
                            num_arr[new_row as usize][new_col as usize] += 1;
                        }
                    }
                }
            }
        }
    }

    let mut total_count = 0;

    loop {
        let mut removed_this_round = 0;
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        // Find all toilet rolls that can be removed this round
        for row in 0..rows {
            for col in 0..cols {
                if vals[row][col] == '@' && num_arr[row][col] < 4 {
                    to_remove.push((row, col));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (row, col) in to_remove {
            vals[row][col] = '.'; // Mark as removed
            num_arr[row][col] = -1;
            removed_this_round += 1;

            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    if new_row >= 0
                        && new_row < rows as i32
                        && new_col >= 0
                        && new_col < cols as i32
                    {
                        num_arr[new_row as usize][new_col as usize] -= 1;
                    }
                }
            }
        }

        total_count += removed_this_round;
    }

    Ok(total_count)
}
