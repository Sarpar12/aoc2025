use std::fs;

fn read_input(input: &str) -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string(input)?;
    let vals: Vec<String> = contents
        .lines()
        .map(|term| term.trim().to_string())
        .filter(|term| !term.is_empty())
        .collect();
    Ok(vals)
}

pub fn part1(input: &str) -> Result<i32, std::io::Error> {
    let mut sum = 0;
    let vals = read_input(input)?;
    for num in vals {
        let chars: Vec<char> = num.chars().collect();

        let mut a = chars[0].to_digit(10).unwrap() as i32;
        let mut b: i32 = -1;

        for i in 1..chars.len() - 1 {
            let d = chars[i].to_digit(10).unwrap() as i32;
            if d > a {
                a = d;
                b = -1;
            } else if d > b {
                b = d;
            }
        }

        let last_digit = chars[chars.len() - 1].to_digit(10).unwrap() as i32;
        if b == -1 {
            b = last_digit;
        } else if last_digit > b {
            b = last_digit;
        }

        sum += (a * 10) + b;
    }
    Ok(sum)
}

pub fn part2(input: &str) -> Result<i64, std::io::Error> {
    let mut sum: i64 = 0;
    let vals = read_input(input)?;
    for num in vals {
        let digits: Vec<i32> = num
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let n = digits.len();
        let k = 12;
        let mut drops_allowed = n - k; 

        let mut batteries: Vec<i32> = Vec::with_capacity(k);

        for d in digits {
            while !batteries.is_empty() && d > *batteries.last().unwrap() && drops_allowed > 0 {
                batteries.pop();
                drops_allowed -= 1;
            }
            if batteries.len() < k {
                batteries.push(d);
            } else {
                drops_allowed -= 1;
            }
        }

        let mut result: i64 = 0;
        for b in batteries.iter() {
            result = result * 10 + (*b as i64);
        }
        sum += result;
    }
    Ok(sum)
}
