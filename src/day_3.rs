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
