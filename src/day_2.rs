use std::fs;

fn read_input(input: &str) -> Result<Vec<(i64, i64)>, std::io::Error> {
    let contents = fs::read_to_string(input)?;
    let vals: Vec<(i64, i64)> = contents
        .split(',')
        .map(|term| {
            let parts: Vec<&str> = term.trim().split('-').collect();
            let a = parts[0].parse::<i64>().unwrap();
            let b = parts[1].parse::<i64>().unwrap();
            (a, b)
        })
        .collect();
    Ok(vals)
}

/// sums all numbers that contains two copies of a number inside
/// ie: 11, 22, 1212
pub fn part1(input: &str) -> Result<i64, std::io::Error> {
    let mut sum: i64 = 0;
    let vals = read_input(input)?;
    for (start, end) in vals {
        for num in start..=end {
            let digits = num.ilog10() + 1;
            if digits % 2 != 0 {
                continue;
            }
            let half_digits = digits / 2;
            let divisor = 10i64.pow(half_digits);
            let second_half = num % divisor;

            if (num / divisor) == second_half {
                sum += num;
            }
        }
    }
    Ok(sum)
}

pub fn part2(input: &str) {}
