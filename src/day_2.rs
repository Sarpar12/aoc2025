use std::fs;

#[allow(dead_code)]
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
#[allow(dead_code)]
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

/// new rule: any number with a repeating sequence >= 2x
/// is valid, sum those
#[allow(dead_code)]
pub fn part2(input: &str) -> Result<i64, std::io::Error> {
    let mut sum = 0;
    let vals = read_input(input)?;
    for (start, end) in vals {
        'num_loop: for num in start..=end {
            let digits = num.ilog10() + 1;
            let half_digits = digits / 2;
            'len_loop: for i in 1..=half_digits {
                if digits % i != 0 {
                    continue; // pattern length must divide evenly into total digits
                }
                let divisor = 10i64.pow(i);
                let mut remaining = num;
                let s1 = remaining % divisor;
                remaining /= divisor;

                while remaining > 0 {
                    let s2 = remaining % divisor;
                    if s1 != s2 {
                        continue 'len_loop; // pattern broke, try next length
                    }
                    remaining /= divisor;
                }
                sum += num;
                continue 'num_loop;
            }
        }
    }
    Ok(sum)
}
