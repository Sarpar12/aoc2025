use std::io::BufRead;
use std::{fs, io};

/*
 * Day 1 Rules:
 * Dial Max: 99(rolls over to 0 when turning right)
 * Dial Min: 0(rolls over to 99 when turning left)
 * Dial Start: 50
 */

/// Part 1 of Day 1
///
/// Returns the number of times the dial reaches 0
///
/// # Arguments
///
/// * `input` - the path of the input file to read
pub fn part1(input: &str) -> Result<i32, std::io::Error> {
    let mut dial = 50;
    let mut count = 0;
    let file = fs::File::open(input)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.chars().nth(0) == Some('L') {
            dial = (dial + 100 - (line[1..].parse::<i32>().unwrap() % 100)) % 100;
        } else if line.chars().nth(0) == Some('R') {
            dial = (dial + 100 + (line[1..].parse::<i32>().unwrap() % 100)) % 100;
        } else {
            panic!("Invalid Input Line");
        }

        if dial == 0 {
            count = count + 1;
        }
    }

    Ok(count)
}

pub fn part2(input: &str) -> () {
    return;
}
