use std::fs;

/*
 * Day 1 Rules:
 * Dial Max: 99(rolls over to 0 when turning right)
 * Dial Min: 0(rolls over to 99 when turning left)
 * Dial Start: 50
 */

/// Reads and parses the input file into a vector of (direction, amount) tuples
/// Direction is true for Right, false for Left
fn read_input(input: &str) -> Result<Vec<(bool, i32)>, std::io::Error> {
    let contents = fs::read_to_string(input)?;
    let instructions: Vec<(bool, i32)> = contents
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let direction = bytes[0] == b'R';
            let amount = line[1..].parse::<i32>().unwrap();
            (direction, amount)
        })
        .collect();
    Ok(instructions)
}

/// Part 1 of Day 1
///
/// Returns the number of times the dial reaches 0 AFTER a
/// rotation has finished
///
/// # Arguments
///
/// * `input` - the path of the input file to read
pub fn part1(input: &str) -> Result<i32, std::io::Error> {
    let instructions = read_input(input)?;
    let mut dial = 50;
    let mut count = 0;

    for (is_right, amount) in instructions {
        let partial = amount % 100;
        if is_right {
            dial = (dial + partial) % 100;
        } else {
            dial = (dial + 100 - partial) % 100;
        }

        if dial == 0 {
            count += 1;
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
    let instructions = read_input(input)?;
    let mut dial = 50;
    let mut count = 0;

    for (is_right, amount) in instructions {
        let full_rotations = amount / 100;
        let partial_amount = amount % 100;

        if is_right {
            if dial != 0 && dial + partial_amount > 99 {
                count += 1;
            }
            dial = (dial + partial_amount) % 100;
        } else {
            if dial != 0 && dial - partial_amount <= 0 {
                count += 1;
            }
            dial = (dial + 100 - partial_amount) % 100;
        }

        count += full_rotations;
    }

    Ok(count)
}
