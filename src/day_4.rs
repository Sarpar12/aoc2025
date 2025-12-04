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

    // Initialize num_arr with same dimensions as vals, filled with 0s
    let mut num_arr: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

    // For each '@' in vals, add 1 to adjacent cells in num_arr
    for row in 0..rows {
        for col in 0..cols {
            if vals[row][col] == '@' {
                // Check all 8 adjacent cells
                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        if dr == 0 && dc == 0 {
                            continue; // Skip the cell itself
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

    // Debug print num_arr
    println!("num_arr:");
    for row in &num_arr {
        println!("{:?}", row);
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
