use std::collections::HashSet;
use std::fs;

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    let contents = std::fs::read_to_string(input).unwrap();
    let normalized = contents.replace("\r\n", "\n");
    let mut lines = normalized.lines();
    let mut beams = HashSet::new();
    beams.insert(lines.next().unwrap().find('S').unwrap());
    let mut split_count = 0;

    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut next_beams = HashSet::new();
        for &x in &beams {
            if line.as_bytes()[x] == b'^' {
                split_count += 1;
                if x > 0 {
                    next_beams.insert(x - 1);
                }
                if x + 1 < line.len() {
                    next_beams.insert(x + 1);
                }
            } else {
                next_beams.insert(x);
            }
        }
        beams = next_beams;
    }
    split_count
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u128 {
    let contents = fs::read_to_string(input).unwrap();
    let normalized = contents.replace("\r\n", "\n");
    let lines: Vec<&str> = normalized.lines().collect();

    // Find the row and column of S
    let start_row = lines.iter().position(|l| l.contains('S')).unwrap();
    let start_col = lines[start_row].find('S').unwrap();

    // Initialize a single vector: number of paths reaching each column
    let width = lines[0].len();
    let mut counts = vec![0u128; width];
    counts[start_col] = 1;

    for line in lines.iter().skip(start_row + 1) {
        if line.is_empty() {
            break;
        }
        let mut next_counts = vec![0u128; width];

        for x in 0..width {
            let c = counts[x];
            if c == 0 {
                continue;
            }
            if line.as_bytes()[x] == b'^' {
                if x > 0 {
                    next_counts[x - 1] += c;
                }
                if x + 1 < width {
                    next_counts[x + 1] += c;
                }
            } else {
                next_counts[x] += c;
            }
        }
        counts = next_counts;
    }
    counts.iter().sum()
}
