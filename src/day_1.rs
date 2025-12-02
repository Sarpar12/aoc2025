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
/// Returns the number of times the dial reaches 0 AFTER a
/// rotation has finished
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

/// Counts EVERY time the dial passes 0, even during a rotation
///
/// # Arguments
///
/// * 'input' - the filename
///
pub fn part2(input: &str) -> Result<i32, std::io::Error> {
    let mut dial = 50;
    let mut count = 0;
    let file = fs::File::open(input)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let amount = line[1..].parse::<i32>().unwrap();
        let full_rotations = amount / 100;
        let partial_amount = amount % 100;

        if line.chars().nth(0) == Some('L') {
            if dial != 0 && dial - partial_amount <= 0 {
                count += 1;
            }
            dial = (dial + 100 - partial_amount) % 100;
        } else if line.chars().nth(0) == Some('R') {
            if dial != 0 && dial + partial_amount > 99 {
                count += 1;
            }
            dial = (dial + partial_amount) % 100;
        } else {
            panic!("Invalid Input Line");
        }

        count += full_rotations;
    }

    Ok(count)
}
